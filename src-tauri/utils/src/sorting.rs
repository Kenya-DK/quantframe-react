use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortQueryDto {
    /// The field to sort by
    pub sort_by: String,
    /// The direction of the sort
    pub sort_direction: SortDirection,
}
impl SortQueryDto {
    pub fn new(sort_by: String, sort_direction: SortDirection) -> Self {
        Self {
            sort_by,
            sort_direction,
        }
    }
}
pub fn sort_data<T>(data: &mut [T], by: String, direction: SortDirection)
where
    T: Ord + Clone,
{
    match direction {
        SortDirection::Asc => data.sort_by(|a, b| {
            let a_key = get_sort_key(a, &by);
            let b_key = get_sort_key(b, &by);
            a_key.cmp(&b_key)
        }),
        SortDirection::Desc => data.sort_by(|a, b| {
            let a_key = get_sort_key(a, &by);
            let b_key = get_sort_key(b, &by);
            b_key.cmp(&a_key)
        }),
    }
}

fn get_sort_key<T>(item: &T, _by: &str) -> T
where
    T: Ord + Clone,
{
    item.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_data_asc() {
        let mut data = vec!["apple", "banana", "cherry"];
        let sort_query = SortQueryDto {
            sort_by: "name".to_string(),
            sort_direction: SortDirection::Asc,
        };
        sort_data(
            &mut data,
            sort_query.sort_by.clone(),
            sort_query.sort_direction.clone(),
        );
        assert_eq!(data, vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_sort_data_desc() {
        let mut data = vec!["apple", "banana", "cherry"];
        let sort_query = SortQueryDto {
            sort_by: "name".to_string(),
            sort_direction: SortDirection::Desc,
        };
        sort_data(
            &mut data,
            sort_query.sort_by.clone(),
            sort_query.sort_direction.clone(),
        );
        assert_eq!(data, vec!["cherry", "banana", "apple"]);
    }
}
