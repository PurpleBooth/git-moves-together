use std::collections::{BTreeMap, BTreeSet};
use std::ops::Div;

use partial_application::partial;

use crate::repository::interface::{ChangeDelta, ChangedFilePath};
use comfy_table::Table;
use std::fmt::{Display, Formatter};

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

impl Statistics {
    pub(crate) fn add_delta(self, delta: &ChangeDelta) -> Statistics {
        Statistics {
            change_deltas: self
                .change_deltas
                .into_iter()
                .chain(vec![delta.clone()].into_iter())
                .collect(),
        }
    }

    pub(crate) fn coupling(&self) -> BTreeMap<CouplingKey, f64> {
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
        accumulator: BTreeMap<CouplingKey, f64>,
        item: &ChangedFilePath,
    ) -> BTreeMap<CouplingKey, f64> {
        let total_changes = self.number_of_deltas_containing(item);

        self.files_to_analyse()
            .iter()
            .filter(partial!(ChangedFilePath::ne => item, _))
            .map(partial!(Statistics::number_of_deltas_containing_both => self, item, _))
            .fold(
                accumulator,
                partial!(Statistics::hash_map_with_new_coupling_item => total_changes, _, _),
            )
    }

    fn hash_map_with_new_coupling_item(
        total_changes: f64,
        acc: BTreeMap<CouplingKey, f64>,
        (coupling_key, count): (CouplingKey, f64),
    ) -> BTreeMap<CouplingKey, f64> {
        acc.into_iter()
            .chain(vec![(coupling_key, (count.div(total_changes)))])
            .filter(|(_, score)| score > &0.0)
            .collect::<BTreeMap<_, _>>()
    }

    #[allow(clippy::cast_precision_loss)]
    fn number_of_deltas_containing_both(
        &self,
        item: &ChangedFilePath,
        other_file: &ChangedFilePath,
    ) -> (CouplingKey, f64) {
        (
            CouplingKey::new(item.clone(), other_file.clone()),
            self.change_deltas
                .clone()
                .into_iter()
                .filter(partial!(Statistics::contains_both => item, other_file, _))
                .count() as f64,
        )
    }

    fn contains_both(
        item: &ChangedFilePath,
        other_file: &ChangedFilePath,
        delta: &ChangeDelta,
    ) -> bool {
        delta.contains(item) && delta.contains(other_file)
    }

    #[allow(clippy::cast_precision_loss)]
    fn number_of_deltas_containing(&self, item: &ChangedFilePath) -> f64 {
        self.change_deltas
            .iter()
            .filter(|delta| delta.contains(item))
            .count() as f64
    }
}

impl Display for Statistics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut coupling: Vec<_> = self.coupling().into_iter().collect();
        coupling.sort_by(|(_, a), (_, b)| (*a).partial_cmp(b).unwrap());
        coupling.reverse();

        let mut table = Table::new();

        table.set_header(vec!["File A", "File B", "Moves Together"]);
        for (key, strength) in coupling {
            table.add_row(vec![
                key.0.into(),
                key.1.into(),
                format!("{:.2}%", strength * 100.0),
            ]);
        }

        writeln!(f, "{}", table)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::repository::interface::ChangeDelta;
    use crate::statistics::{CouplingKey, Statistics};

    #[test]
    fn adding_one_file_to_statistics_will_give_a_count_of_zero() {
        let statistics = Statistics::default();
        let actual = statistics.add_delta(&ChangeDelta::from(vec!["file_1"]));
        assert_eq!(actual.coupling(), BTreeMap::new());
    }

    #[test]
    fn a_file_two_files_at_the_same_time_twice_is_full_coupling() {
        let statistics = Statistics::default();
        let actual = statistics
            .add_delta(&ChangeDelta::from(vec!["file_1", "file_2"]))
            .add_delta(&ChangeDelta::from(vec!["file_1", "file_2"]))
            .add_delta(&ChangeDelta::from(vec!["file_1", "file_2"]));
        assert_eq!(
            actual.coupling(),
            vec![(CouplingKey::new("file_1".into(), "file_2".into()), 1.0),]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn more_complex_coupling() {
        let statistics = Statistics::default();
        let actual = statistics
            .add_delta(&ChangeDelta::from(vec!["file_1", "file_2"]))
            .add_delta(&ChangeDelta::from(vec!["file_3", "file_2"]))
            .add_delta(&ChangeDelta::from(vec!["file_3", "file_2"]))
            .add_delta(&ChangeDelta::from(vec!["file_3", "file_5"]))
            .add_delta(&ChangeDelta::from(vec!["file_3", "file_1"]))
            .add_delta(&ChangeDelta::from(vec!["file_1", "file_2"]));
        assert_eq!(
            actual.coupling(),
            vec![
                (CouplingKey::new("file_1".into(), "file_2".into()), 0.5),
                (CouplingKey::new("file_1".into(), "file_3".into()), 0.25),
                (CouplingKey::new("file_2".into(), "file_3".into()), 0.5),
                (CouplingKey::new("file_3".into(), "file_5".into()), 1.0)
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn statistics_render_pretty() {
        let statistics = Statistics::default()
            .add_delta(&ChangeDelta::from(vec!["file_1", "file_2"]))
            .add_delta(&ChangeDelta::from(vec!["file_3", "file_2"]))
            .add_delta(&ChangeDelta::from(vec!["file_3", "file_2"]))
            .add_delta(&ChangeDelta::from(vec!["file_3", "file_5"]))
            .add_delta(&ChangeDelta::from(vec!["file_3", "file_1"]))
            .add_delta(&ChangeDelta::from(vec!["file_1", "file_2"]));
        assert_eq!(
            format!("{}", statistics),
            "+--------+--------+----------------+
| File A | File B | Moves Together |
+==================================+
| file_3 | file_5 | 100.00%        |
|--------+--------+----------------|
| file_2 | file_3 | 50.00%         |
|--------+--------+----------------|
| file_1 | file_2 | 50.00%         |
|--------+--------+----------------|
| file_1 | file_3 | 25.00%         |
+--------+--------+----------------+
"
        );
    }
}
