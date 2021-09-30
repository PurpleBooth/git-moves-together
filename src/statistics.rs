use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fmt::{Display, Formatter},
};

use chrono::{Duration, DurationRound};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, ContentArrangement, Table};

use crate::model::{changed_file::ChangedFile, delta::Delta, hash::Hash};

#[derive(Eq, PartialEq, Hash, Debug, Ord, PartialOrd, Clone)]
pub(crate) struct Key {
    left: ChangedFile,
    right: ChangedFile,
}

impl Key {
    fn new(left: ChangedFile, right: ChangedFile) -> Key {
        let mut keys = [left, right];
        keys.sort();

        Key {
            left: keys.get(0).unwrap().clone(),
            right: keys.get(1).unwrap().clone(),
        }
    }
}

pub(crate) struct CouplingResult {
    result: Vec<(Key, Calculation)>,
}

impl CouplingResult {
    pub(crate) fn is_empty(&self) -> bool {
        self.result.is_empty()
    }
}

#[derive(Default)]
pub(crate) struct Statistics {
    hash_to_delta: BTreeMap<Hash, Delta>,
    change_to_delta: BTreeMap<ChangedFile, BTreeSet<Delta>>,
}

type Calculation = (f64, usize, usize);

#[allow(clippy::cast_precision_loss)]
fn display_order((_, a): &(Key, Calculation), (_, b): &(Key, Calculation)) -> Ordering {
    (a.0 * (a.2 as f64))
        .partial_cmp(&(b.0 * (b.2 as f64)))
        .unwrap()
}

#[derive(Copy, Clone)]
pub enum Strategy {
    Hash,
    CommitTime(Duration),
}

impl Statistics {
    pub(crate) async fn add_delta(self, delta: Delta, strategy: &Strategy) -> Statistics {
        let mut hash_to_delta = self.hash_to_delta;
        let (key, grouped_delta) = match strategy {
            Strategy::Hash => (delta.hash(), delta.clone()),
            Strategy::CommitTime(duration) => {
                let key: Hash = (&delta)
                    .timestamp()
                    .duration_trunc(*duration)
                    .unwrap()
                    .into();
                (
                    key.clone(),
                    match hash_to_delta.get(&key) {
                        None => delta.clone(),
                        Some(existing_delta) => existing_delta.merge(&delta),
                    },
                )
            }
        };
        hash_to_delta.insert(key, grouped_delta.clone());

        let mut change_to_delta = self.change_to_delta;
        for change in grouped_delta {
            let mut coupled_deltas = match change_to_delta.get(&change) {
                None => BTreeSet::new(),
                Some(coupled_delta) => coupled_delta.clone(),
            };

            coupled_deltas.insert(delta.clone());
            change_to_delta.insert(change, coupled_deltas);
        }

        Statistics {
            hash_to_delta,
            change_to_delta,
        }
    }

    pub(crate) fn coupling(&self) -> CouplingResult {
        let changes = self.changed_files();
        return CouplingResult {
            result: changes
                .iter()
                .fold(BTreeMap::new(), |total, change| {
                    self.add_statistic(&changes, total, change)
                })
                .into_iter()
                .collect(),
        };
    }

    fn changed_files(&self) -> BTreeSet<ChangedFile> {
        self.hash_to_delta
            .iter()
            .flat_map(|(_, change_delta)| change_delta.clone())
            .collect()
    }

    fn add_statistic(
        &self,
        changes: &BTreeSet<ChangedFile>,
        total: BTreeMap<Key, Calculation>,
        change: &ChangedFile,
    ) -> BTreeMap<Key, Calculation> {
        changes
            .iter()
            .filter(|other| &change != other)
            .map(|other| self.deltas_containing(change, other))
            .fold(total, |acc, count_and_total| {
                Statistics::insert_with_new_coupling_item(acc, count_and_total)
            })
    }

    #[allow(clippy::cast_precision_loss)]
    fn insert_with_new_coupling_item(
        acc: BTreeMap<Key, Calculation>,
        (coupling_key, count, total_changes): (Key, usize, usize),
    ) -> BTreeMap<Key, Calculation> {
        let mut new = acc;
        let score = (count as f64) / (total_changes as f64);

        if score > 0.0 {
            new.insert(coupling_key, (score, count, total_changes));
        }

        new
    }

    fn deltas_containing(
        &self,
        item: &ChangedFile,
        other_file: &ChangedFile,
    ) -> (Key, usize, usize) {
        (
            Key::new(item.clone(), other_file.clone()),
            self.deltas_containing_both(item, other_file),
            self.deltas_containing_either(item, other_file),
        )
    }

    fn deltas_containing_both(&self, item: &ChangedFile, other_file: &ChangedFile) -> usize {
        self.change_to_delta
            .get(item)
            .zip(self.change_to_delta.get(other_file))
            .map_or(0, |(left, right)| left.intersection(right).count())
    }

    fn deltas_containing_either(&self, item: &ChangedFile, other_file: &ChangedFile) -> usize {
        self.change_to_delta
            .get(item)
            .zip(self.change_to_delta.get(other_file))
            .map_or(0, |(left, right)| left.union(right).count())
    }
}

impl Display for CouplingResult {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut coupling: Vec<_> = self.result.clone();
        coupling.sort_by(display_order);

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                "File A",
                "File B",
                "Together %",
                "Together",
                "Commits",
            ]);
        for (key, (strength, together, total)) in coupling {
            table.add_row(vec![
                key.left.clone().into(),
                key.right.clone().into(),
                format!("{:.2}%", strength * 100.0),
                format!("{}", together),
                format!("{}", total),
            ]);
        }

        writeln!(formatter, "{}", table)
    }
}

#[cfg(test)]
mod tests {

    use chrono::Utc;

    use crate::{
        model::delta::Delta,
        statistics::{Key, Statistics, Strategy},
    };

    #[allow(clippy::semicolon_if_nothing_returned)]
    #[tokio::test]
    async fn adding_one_file_to_statistics_will_give_a_count_of_zero() {
        let statistics = Statistics::default();
        let actual = statistics.add_delta(
            Delta::new("Id".into(), Utc::now(), vec!["file_1".into()]),
            &Strategy::Hash,
        );
        assert_eq!(actual.await.coupling().result, Vec::new());
    }

    #[allow(clippy::semicolon_if_nothing_returned)]
    #[tokio::test]
    async fn a_file_two_files_at_the_same_time_twice_is_full_coupling() {
        let statistics = Statistics::default();
        let actual = statistics
            .add_delta(
                Delta::new(
                    "1".into(),
                    Utc::now(),
                    vec!["file_1".into(), "file_2".into()],
                ),
                &Strategy::Hash,
            )
            .await
            .add_delta(
                Delta::new(
                    "2".into(),
                    Utc::now(),
                    vec!["file_1".into(), "file_2".into()],
                ),
                &Strategy::Hash,
            )
            .await
            .add_delta(
                Delta::new(
                    "3".into(),
                    Utc::now(),
                    vec!["file_1".into(), "file_2".into()],
                ),
                &Strategy::Hash,
            );
        assert_eq!(
            actual.await.coupling().result,
            vec![(Key::new("file_1".into(), "file_2".into()), (1.0, 3, 3)),]
        );
    }

    #[allow(clippy::semicolon_if_nothing_returned)]
    #[tokio::test]
    async fn more_complex_coupling() {
        let statistics = Statistics::default();
        let actual = statistics
            .add_delta(
                Delta::new(
                    "1".into(),
                    Utc::now(),
                    vec!["file_1".into(), "file_2".into()],
                ),
                &Strategy::Hash,
            )
            .await
            .add_delta(
                Delta::new(
                    "2".into(),
                    Utc::now(),
                    vec!["file_3".into(), "file_2".into()],
                ),
                &Strategy::Hash,
            )
            .await
            .add_delta(
                Delta::new(
                    "3".into(),
                    Utc::now(),
                    vec!["file_3".into(), "file_2".into()],
                ),
                &Strategy::Hash,
            )
            .await
            .add_delta(
                Delta::new(
                    "4".into(),
                    Utc::now(),
                    vec!["file_3".into(), "file_5".into()],
                ),
                &Strategy::Hash,
            )
            .await
            .add_delta(
                Delta::new(
                    "5".into(),
                    Utc::now(),
                    vec!["file_3".into(), "file_1".into()],
                ),
                &Strategy::Hash,
            )
            .await
            .add_delta(
                Delta::new(
                    "6".into(),
                    Utc::now(),
                    vec!["file_1".into(), "file_2".into()],
                ),
                &Strategy::Hash,
            );
        assert_eq!(
            actual.await.coupling().result,
            vec![
                (Key::new("file_1".into(), "file_2".into()), (0.4, 2, 5)),
                (
                    Key::new("file_1".into(), "file_3".into()),
                    (0.166_666_666_666_666_66, 1, 6)
                ),
                (
                    Key::new("file_2".into(), "file_3".into()),
                    (0.333_333_333_333_333_3, 2, 6)
                ),
                (Key::new("file_3".into(), "file_5".into()), (0.25, 1, 4)),
            ]
        );
    }

    #[allow(clippy::semicolon_if_nothing_returned)]
    #[tokio::test]
    async fn statistics_render_pretty() {
        let statistics = Statistics::default()
            .add_delta(
                Delta::new(
                    "1".into(),
                    Utc::now(),
                    vec!["file_1".into(), "file_2".into()],
                ),
                &Strategy::Hash,
            )
            .await
            .add_delta(
                Delta::new(
                    "2".into(),
                    Utc::now(),
                    vec!["file_3".into(), "file_2".into()],
                ),
                &Strategy::Hash,
            )
            .await
            .add_delta(
                Delta::new(
                    "3".into(),
                    Utc::now(),
                    vec!["file_3".into(), "file_2".into()],
                ),
                &Strategy::Hash,
            )
            .await
            .add_delta(
                Delta::new(
                    "4".into(),
                    Utc::now(),
                    vec!["file_3".into(), "file_5".into()],
                ),
                &Strategy::Hash,
            )
            .await
            .add_delta(
                Delta::new(
                    "5".into(),
                    Utc::now(),
                    vec!["file_3".into(), "file_1".into()],
                ),
                &Strategy::Hash,
            )
            .await
            .add_delta(
                Delta::new(
                    "6".into(),
                    Utc::now(),
                    vec!["file_1".into(), "file_2".into()],
                )
                .add_str_prefix("demo"),
                &Strategy::Hash,
            );
        assert_eq!(
            format!("{}", statistics.await.coupling()),
            "\u{256d}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{252c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{252c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{252c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{252c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{256e}\n\u{2502} File A      \u{2506} File B      \u{2506} Together % \u{2506} Together \u{2506} Commits \u{2502}\n\u{255e}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{256a}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{256a}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{256a}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{256a}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2561}\n\u{2502} file_1      \u{2506} file_2      \u{2506} 25.00%     \u{2506} 1        \u{2506} 4       \u{2502}\n\u{251c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{253c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{253c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{253c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{253c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{2524}\n\u{2502} file_1      \u{2506} file_3      \u{2506} 20.00%     \u{2506} 1        \u{2506} 5       \u{2502}\n\u{251c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{253c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{253c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{253c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{253c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{2524}\n\u{2502} file_3      \u{2506} file_5      \u{2506} 25.00%     \u{2506} 1        \u{2506} 4       \u{2502}\n\u{251c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{253c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{253c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{253c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{253c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{2524}\n\u{2502} demo@file_1 \u{2506} demo@file_2 \u{2506} 100.00%    \u{2506} 1        \u{2506} 1       \u{2502}\n\u{251c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{253c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{253c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{253c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{253c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{2524}\n\u{2502} file_2      \u{2506} file_3      \u{2506} 40.00%     \u{2506} 2        \u{2506} 5       \u{2502}\n\u{2570}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2534}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2534}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2534}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2534}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{256f}\n"
        );
    }
}
