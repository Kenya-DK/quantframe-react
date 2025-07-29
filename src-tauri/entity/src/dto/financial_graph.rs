use std::collections::HashMap;

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialGraph<N> {
    pub labels: Vec<String>,
    pub values: Vec<N>,
}

impl<N> FinancialGraph<N>
where
    N: Default + Copy,
{
    pub fn new(labels: Vec<String>, values: Vec<N>) -> Self {
        Self { labels, values }
    }

    pub fn from<T, F>(grouped: &HashMap<String, Vec<T>>, extract_value: F) -> Self
    where
        T: Clone,
        N: Copy + Ord,
        F: Fn(&[T]) -> N,
    {
        let mut entries: Vec<(String, N)> = grouped
            .iter()
            .map(|(key, group)| (key.clone(), extract_value(group)))
            .collect();

        entries.sort_by_key(|(key, _)| key.clone());

        let (labels, values): (Vec<_>, Vec<_>) = entries.into_iter().unzip();

        FinancialGraph::new(labels, values)
    }
}
