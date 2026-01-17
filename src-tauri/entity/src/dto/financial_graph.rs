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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialGraphMap<N> {
    pub labels: Vec<String>,
    pub values: HashMap<String, Vec<N>>,
}
impl<N> FinancialGraphMap<N>
where
    N: Copy,
{
    pub fn new(labels: Vec<String>, values: HashMap<String, Vec<N>>) -> Self {
        Self { labels, values }
    }
    pub fn from<T, F>(grouped: &HashMap<String, Vec<T>>, extract: F) -> Self
    where
        T: Clone,
        F: Fn(&[T]) -> HashMap<&str, N>,
    {
        // 1. Sort labels
        let mut entries: Vec<(&String, &Vec<T>)> = grouped.iter().collect();
        entries.sort_by_key(|(k, _)| *k);

        let mut labels = Vec::with_capacity(entries.len());
        let mut values: HashMap<String, Vec<N>> = HashMap::new();

        // 2. Transpose HashMap<String, N> → HashMap<String, Vec<N>>
        for (label, group) in entries {
            labels.push(label.clone());

            let extracted = extract(group);

            for (series, value) in extracted {
                values.entry(series.to_string()).or_default().push(value);
            }
        }

        Self { labels, values }
    }
}
