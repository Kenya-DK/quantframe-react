use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OperationSet {
    pub operations: Vec<String>,
}

impl OperationSet {
    pub fn new() -> Self {
        OperationSet {
            operations: Vec::new(),
        }
    }
    pub fn add(&mut self, operation: impl Into<String>) {
        let operation = operation.into();
        if !self.has(&operation) {
            self.operations.push(operation);
        }
    }
    pub fn has(&self, operation: impl Into<String>) -> bool {
        let operation = operation.into();
        self.operations.iter().any(|op| op == &operation)
    }

    pub fn ends_with(&self, suffix: impl Into<String>) -> bool {
        let suffix = suffix.into();
        self.operations.iter().any(|op| op.ends_with(&suffix))
    }
    pub fn any(&self, operations: &[&str]) -> bool {
        operations.iter().any(|op| self.has(op.to_string()))
    }
    pub fn merge(&mut self, other: &OperationSet) {
        for op in &other.operations {
            self.add(op.clone());
        }
    }
    pub fn set(&mut self, operations: &[&str]) {
        self.operations = operations.iter().map(|&s| s.to_string()).collect();
    }
    pub fn get_value_after(&self, prefix: impl Into<String>) -> Option<String> {
        let prefix = prefix.into();
        let prefix_with_colon = format!("{}:", prefix);

        self.operations
            .iter()
            .find(|op| op.starts_with(&prefix_with_colon))
            .and_then(|op| {
                op.strip_prefix(&prefix_with_colon)
                    .map(|value| value.trim().to_string())
            })
            .filter(|value| !value.is_empty())
    }
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
}

impl Default for OperationSet {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for OperationSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.operations)
    }
}
