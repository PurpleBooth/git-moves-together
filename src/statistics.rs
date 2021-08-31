use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter};

use comfy_table::Table;

use crate::model::change_delta::ChangeDelta;
use crate::model::changed_file_path::ChangedFilePath;
use crate::model::snapshot_id::SnapshotId;
use chrono::Duration;
use chrono::DurationRound;

#[derive(Eq, PartialEq, Hash, Debug, Ord, PartialOrd, Clone)]
pub(crate) struct CouplingKey(ChangedFilePath, ChangedFilePath);

impl CouplingKey {
    fn new(first_path: ChangedFilePath, second_path: ChangedFilePath) -> CouplingKey {
        let mut keys = [first_path, second_path];
        keys.sort();

        CouplingKey(keys.get(0).unwrap().clone(), keys.get(1).unwrap().clone())
    }
}

#[derive(Default)]
pub(crate) struct Statistics {
    change_deltas: BTreeMap<SnapshotId, ChangeDelta>,
    contains: BTreeMap<ChangedFilePath, BTreeSet<ChangeDelta>>,
}

type CouplingCalculation = (f64, usize, usize);

#[allow(clippy::cast_precision_loss)]
fn coupling_calc_rank(
    (_, a): &(CouplingKey, CouplingCalculation),
    (_, b): &(CouplingKey, CouplingCalculation),
) -> Ordering {
    (a.0 * (a.2 as f64))
        .partial_cmp(&(b.0 * (b.2 as f64)))
        .unwrap()
}

#[derive(Copy, Clone)]
pub enum Strategy {
    Id,
    CommitTime(Duration),
}

impl Statistics {
    pub(crate) fn add_delta(self, delta: &ChangeDelta, strategy: &Strategy) -> Statistics {
        let mut change_map = self.change_deltas;
        let (key, new_delta) = match strategy {
            Strategy::Id => (delta.id(), delta.clone()),
            Strategy::CommitTime(duration) => {
                let key: SnapshotId = delta
                    .timestamp()
                    .duration_trunc(*duration)
                    .unwrap()
                    .to_string()
                    .into();
                (
                    key.clone(),
                    match change_map.get(&key) {
                        None => delta.clone(),
                        Some(existing_delta) => existing_delta.merge(delta),
                    },
                )
            }
        };
        change_map.insert(key, new_delta.clone());

        let mut map = self.contains;
        for change in new_delta {
            let mut current_deltas = match map.get(&change) {
                None => BTreeSet::new(),
                Some(set) => set.clone(),
            };

            current_deltas.insert(delta.clone());

            map.insert(change, current_deltas);
        }

        Statistics {
            change_deltas: change_map,
            contains: map,
        }
    }

    pub(crate) fn coupling(&self) -> BTreeMap<CouplingKey, CouplingCalculation> {
        let set = self.files_to_analyse();
        return set.iter().fold(BTreeMap::new(), |acc, item| {
            self.add_statistic(&set, acc, item)
        });
    }

    fn files_to_analyse(&self) -> BTreeSet<ChangedFilePath> {
        self.change_deltas
            .iter()
            .flat_map(|(_, change_delta)| change_delta.clone())
            .collect()
    }

    fn add_statistic(
        &self,
        set: &BTreeSet<ChangedFilePath>,
        accumulator: BTreeMap<CouplingKey, CouplingCalculation>,
        item: &ChangedFilePath,
    ) -> BTreeMap<CouplingKey, CouplingCalculation> {
        set.iter()
            .filter(|other| &item != other)
            .map(|other| self.number_of_deltas_containing(item, other))
            .fold(accumulator, |acc, key_count_and_total| {
                Statistics::insert_with_new_coupling_item(acc, key_count_and_total)
            })
    }

    #[allow(clippy::cast_precision_loss)]
    fn insert_with_new_coupling_item(
        acc: BTreeMap<CouplingKey, CouplingCalculation>,
        (coupling_key, count, total_changes): (CouplingKey, usize, usize),
    ) -> BTreeMap<CouplingKey, CouplingCalculation> {
        let mut new = acc;
        let score = (count as f64) / (total_changes as f64);

        if score > 0.0 {
            new.insert(coupling_key, (score, count, total_changes));
        }

        new
    }

    fn number_of_deltas_containing(
        &self,
        item: &ChangedFilePath,
        other_file: &ChangedFilePath,
    ) -> (CouplingKey, usize, usize) {
        (
            CouplingKey::new(item.clone(), other_file.clone()),
            self.number_of_deltas_containing_both(item, other_file),
            self.number_of_deltas_containing_either(item, other_file),
        )
    }

    fn number_of_deltas_containing_both(
        &self,
        item: &ChangedFilePath,
        other_file: &ChangedFilePath,
    ) -> usize {
        self.contains
            .get(item)
            .zip(self.contains.get(other_file))
            .map_or(0, |(left, right)| left.intersection(right).count())
    }

    fn number_of_deltas_containing_either(
        &self,
        item: &ChangedFilePath,
        other_file: &ChangedFilePath,
    ) -> usize {
        self.contains
            .get(item)
            .zip(self.contains.get(other_file))
            .map_or(0, |(left, right)| left.union(right).count())
    }
}

impl Display for Statistics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut coupling: Vec<_> = self.coupling().into_iter().collect();
        coupling.sort_by(coupling_calc_rank);

        let mut table = Table::new();

        table.set_header(vec![
            "File A",
            "File B",
            "Together %",
            "Together",
            "Commits",
        ]);
        for (key, (strength, together, total)) in coupling {
            table.add_row(vec![
                key.0.into(),
                key.1.into(),
                format!("{:.2}%", strength * 100.0),
                format!("{}", together),
                format!("{}", total),
            ]);
        }

        writeln!(f, "{}", table)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use chrono::Utc;

    use crate::model::change_delta::ChangeDelta;
    use crate::statistics::{CouplingKey, Statistics, Strategy};

    #[test]
    fn adding_one_file_to_statistics_will_give_a_count_of_zero() {
        let statistics = Statistics::default();
        let actual = statistics.add_delta(
            &ChangeDelta::new("Id".into(), Utc::now(), vec!["file_1".into()]),
            &Strategy::Id,
        );
        assert_eq!(actual.coupling(), BTreeMap::new());
    }

    #[test]
    fn a_file_two_files_at_the_same_time_twice_is_full_coupling() {
        let statistics = Statistics::default();
        let actual = statistics
            .add_delta(
                &ChangeDelta::new(
                    "1".into(),
                    Utc::now(),
                    vec!["file_1".into(), "file_2".into()],
                ),
                &Strategy::Id,
            )
            .add_delta(
                &ChangeDelta::new(
                    "2".into(),
                    Utc::now(),
                    vec!["file_1".into(), "file_2".into()],
                ),
                &Strategy::Id,
            )
            .add_delta(
                &ChangeDelta::new(
                    "3".into(),
                    Utc::now(),
                    vec!["file_1".into(), "file_2".into()],
                ),
                &Strategy::Id,
            );
        assert_eq!(
            actual.coupling(),
            vec![(
                CouplingKey::new("file_1".into(), "file_2".into()),
                (1.0, 3, 3)
            ),]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn more_complex_coupling() {
        let statistics = Statistics::default();
        let actual = statistics
            .add_delta(
                &ChangeDelta::new(
                    "1".into(),
                    Utc::now(),
                    vec!["file_1".into(), "file_2".into()],
                ),
                &Strategy::Id,
            )
            .add_delta(
                &ChangeDelta::new(
                    "2".into(),
                    Utc::now(),
                    vec!["file_3".into(), "file_2".into()],
                ),
                &Strategy::Id,
            )
            .add_delta(
                &ChangeDelta::new(
                    "3".into(),
                    Utc::now(),
                    vec!["file_3".into(), "file_2".into()],
                ),
                &Strategy::Id,
            )
            .add_delta(
                &ChangeDelta::new(
                    "4".into(),
                    Utc::now(),
                    vec!["file_3".into(), "file_5".into()],
                ),
                &Strategy::Id,
            )
            .add_delta(
                &ChangeDelta::new(
                    "5".into(),
                    Utc::now(),
                    vec!["file_3".into(), "file_1".into()],
                ),
                &Strategy::Id,
            )
            .add_delta(
                &ChangeDelta::new(
                    "6".into(),
                    Utc::now(),
                    vec!["file_1".into(), "file_2".into()],
                ),
                &Strategy::Id,
            );
        assert_eq!(
            actual.coupling(),
            vec![
                (
                    CouplingKey::new("file_1".into(), "file_2".into()),
                    (0.4, 2, 5)
                ),
                (
                    CouplingKey::new("file_1".into(), "file_3".into()),
                    (0.166_666_666_666_666_66, 1, 6)
                ),
                (
                    CouplingKey::new("file_2".into(), "file_3".into()),
                    (0.333_333_333_333_333_3, 2, 6)
                ),
                (
                    CouplingKey::new("file_3".into(), "file_5".into()),
                    (0.25, 1, 4)
                ),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn statistics_render_pretty() {
        let statistics = Statistics::default()
            .add_delta(
                &ChangeDelta::new(
                    "1".into(),
                    Utc::now(),
                    vec!["file_1".into(), "file_2".into()],
                ),
                &Strategy::Id,
            )
            .add_delta(
                &ChangeDelta::new(
                    "2".into(),
                    Utc::now(),
                    vec!["file_3".into(), "file_2".into()],
                ),
                &Strategy::Id,
            )
            .add_delta(
                &ChangeDelta::new(
                    "3".into(),
                    Utc::now(),
                    vec!["file_3".into(), "file_2".into()],
                ),
                &Strategy::Id,
            )
            .add_delta(
                &ChangeDelta::new(
                    "4".into(),
                    Utc::now(),
                    vec!["file_3".into(), "file_5".into()],
                ),
                &Strategy::Id,
            )
            .add_delta(
                &ChangeDelta::new(
                    "5".into(),
                    Utc::now(),
                    vec!["file_3".into(), "file_1".into()],
                ),
                &Strategy::Id,
            )
            .add_delta(
                &ChangeDelta::new(
                    "6".into(),
                    Utc::now(),
                    vec!["file_1".into(), "file_2".into()],
                )
                .add_prefix("demo"),
                &Strategy::Id,
            );
        assert_eq!(
            format!("{}", statistics),
            "+-------------+-------------+------------+----------+---------+
| File A      | File B      | Together % | Together | Commits |
+=============================================================+
| file_1      | file_2      | 25.00%     | 1        | 4       |
|-------------+-------------+------------+----------+---------|
| file_1      | file_3      | 20.00%     | 1        | 5       |
|-------------+-------------+------------+----------+---------|
| file_3      | file_5      | 25.00%     | 1        | 4       |
|-------------+-------------+------------+----------+---------|
| demo@file_1 | demo@file_2 | 100.00%    | 1        | 1       |
|-------------+-------------+------------+----------+---------|
| file_2      | file_3      | 40.00%     | 2        | 5       |
+-------------+-------------+------------+----------+---------+
"
        );
    }
}
