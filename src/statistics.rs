use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter};

use comfy_table::Table;
use partial_application::partial;

use crate::model::change_delta::ChangeDelta;
use crate::model::changed_file_path::ChangedFilePath;

#[derive(Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
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
    change_deltas: Vec<ChangeDelta>,
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

impl Statistics {
    pub(crate) fn add_delta(self, delta: ChangeDelta) -> Statistics {
        Statistics {
            change_deltas: self
                .change_deltas
                .into_iter()
                .chain(vec![delta].into_iter())
                .collect(),
        }
    }

    pub(crate) fn coupling(&self) -> BTreeMap<CouplingKey, CouplingCalculation> {
        return self.files_to_analyse().iter().fold(
            BTreeMap::new(),
            partial!(Statistics::add_statistics_to_hash_map => self, _, _),
        );
    }

    fn files_to_analyse(&self) -> BTreeSet<ChangedFilePath> {
        self.change_deltas.iter().flatten().collect()
    }

    fn add_statistics_to_hash_map(
        &self,
        accumulator: BTreeMap<CouplingKey, CouplingCalculation>,
        item: &ChangedFilePath,
    ) -> BTreeMap<CouplingKey, CouplingCalculation> {
        self.files_to_analyse()
            .iter()
            .filter(partial!(ChangedFilePath::ne => item, _))
            .map(partial!(Statistics::number_of_deltas_containing => self, item, _))
            .fold(accumulator, Statistics::hash_map_with_new_coupling_item)
    }

    #[allow(clippy::cast_precision_loss)]
    fn hash_map_with_new_coupling_item(
        acc: BTreeMap<CouplingKey, CouplingCalculation>,
        (coupling_key, count, total_changes): (CouplingKey, usize, usize),
    ) -> BTreeMap<CouplingKey, CouplingCalculation> {
        acc.into_iter()
            .chain(vec![(
                coupling_key,
                (
                    (count as f64) / (total_changes as f64),
                    count,
                    total_changes,
                ),
            )])
            .filter(|(_, (score, _, _))| score > &0.0)
            .collect::<BTreeMap<_, _>>()
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
        self.change_deltas
            .clone()
            .into_iter()
            .filter(partial!(Statistics::contains_both => item, other_file, _))
            .count()
    }

    fn number_of_deltas_containing_either(
        &self,
        item: &ChangedFilePath,
        other_file: &ChangedFilePath,
    ) -> usize {
        self.change_deltas
            .clone()
            .into_iter()
            .filter(partial!(Statistics::contains_either => item, other_file, _))
            .count()
    }

    fn contains_both(
        item: &ChangedFilePath,
        other_file: &ChangedFilePath,
        delta: &ChangeDelta,
    ) -> bool {
        delta.contains(item) && delta.contains(other_file)
    }

    fn contains_either(
        item: &ChangedFilePath,
        other_file: &ChangedFilePath,
        delta: &ChangeDelta,
    ) -> bool {
        delta.contains(item) || delta.contains(other_file)
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

    use crate::model::change_delta::ChangeDelta;
    use crate::statistics::{CouplingKey, Statistics};

    #[test]
    fn adding_one_file_to_statistics_will_give_a_count_of_zero() {
        let statistics = Statistics::default();
        let actual = statistics.add_delta(ChangeDelta::from(vec!["file_1"]));
        assert_eq!(actual.coupling(), BTreeMap::new());
    }

    #[test]
    fn a_file_two_files_at_the_same_time_twice_is_full_coupling() {
        let statistics = Statistics::default();
        let actual = statistics
            .add_delta(ChangeDelta::from(vec!["file_1", "file_2"]))
            .add_delta(ChangeDelta::from(vec!["file_1", "file_2"]))
            .add_delta(ChangeDelta::from(vec!["file_1", "file_2"]));
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
            .add_delta(ChangeDelta::from(vec!["file_1", "file_2"]))
            .add_delta(ChangeDelta::from(vec!["file_3", "file_2"]))
            .add_delta(ChangeDelta::from(vec!["file_3", "file_2"]))
            .add_delta(ChangeDelta::from(vec!["file_3", "file_5"]))
            .add_delta(ChangeDelta::from(vec!["file_3", "file_1"]))
            .add_delta(ChangeDelta::from(vec!["file_1", "file_2"]));
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
            .add_delta(ChangeDelta::from(vec!["file_1", "file_2"]))
            .add_delta(ChangeDelta::from(vec!["file_3", "file_2"]))
            .add_delta(ChangeDelta::from(vec!["file_3", "file_2"]))
            .add_delta(ChangeDelta::from(vec!["file_3", "file_5"]))
            .add_delta(ChangeDelta::from(vec!["file_3", "file_1"]))
            .add_delta(ChangeDelta::from(vec!["file_1", "file_2"]).add_prefix("demo"));
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
