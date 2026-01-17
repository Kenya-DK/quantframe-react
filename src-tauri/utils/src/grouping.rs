use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum GroupByDate {
    Year,
    Month,
    Day,
    Hour,
}
impl GroupByDate {
    pub fn to_duration(&self) -> Duration {
        match self {
            GroupByDate::Year => Duration::days(365),
            GroupByDate::Month => Duration::days(30),
            GroupByDate::Day => Duration::days(1),
            GroupByDate::Hour => Duration::hours(1),
        }
    }
}
pub fn sort_grouped<T>(grouped: &HashMap<String, T>) -> Vec<(String, T)>
where
    T: Clone,
{
    let mut entries: Vec<(String, T)> = grouped
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    entries.sort_by_key(|(k, _)| k.clone()); // Sort by the date key string
    entries
}
/// Fills in missing date keys between start and end using group levels
pub fn fill_missing_date_keys<T>(
    grouped: &mut HashMap<String, Vec<T>>,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    group_by: &[GroupByDate],
) {
    let mut cursor = start.naive_utc();

    while cursor <= end.naive_utc() {
        let key = generate_date_key(cursor, group_by);
        grouped.entry(key).or_insert_with(Vec::new);

        // Determine step size based on most granular group level
        cursor = match group_by.iter().max() {
            Some(GroupByDate::Hour) => cursor + Duration::hours(1),
            Some(GroupByDate::Day) => cursor + Duration::days(1),
            Some(GroupByDate::Month) => {
                let year = cursor.year();
                let month = cursor.month();
                let next_month = if month == 12 {
                    NaiveDateTime::new(
                        chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap(),
                        chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                    )
                } else {
                    NaiveDateTime::new(
                        chrono::NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap(),
                        chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                    )
                };
                next_month
            }
            Some(GroupByDate::Year) => {
                let next_year = NaiveDateTime::new(
                    chrono::NaiveDate::from_ymd_opt(cursor.year() + 1, 1, 1).unwrap(),
                    chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                );
                next_year
            }
            None => break,
        };
    }
}

pub fn group_by<T, K, F>(items: &[T], key_fn: F) -> HashMap<K, Vec<T>>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
    T: Clone,
{
    let mut map: HashMap<K, Vec<T>> = HashMap::new();
    for item in items {
        let key = key_fn(item);
        map.entry(key).or_default().push(item.clone());
    }
    map
}

pub fn group_by_date<T, F>(
    items: &[T],
    date_extractor: F,
    group_levels: &[GroupByDate],
) -> HashMap<String, Vec<T>>
where
    F: Fn(&T) -> DateTime<Utc>,
    T: Clone,
{
    group_by(items, |item| {
        let dt_local = date_extractor(item)
            .with_timezone(&chrono::Local)
            .naive_local();
        generate_date_key(dt_local, group_levels)
    })
}

fn generate_date_key(dt: NaiveDateTime, levels: &[GroupByDate]) -> String {
    let mut parts = Vec::with_capacity(levels.len());
    for level in levels {
        match level {
            GroupByDate::Year => parts.push(format!("{:04}", dt.year())),
            GroupByDate::Month => parts.push(format!("{:02}", dt.month())),
            GroupByDate::Day => parts.push(format!("{:02}", dt.day())),
            GroupByDate::Hour => parts.push(format!("{:02}:00", dt.hour())),
        }
    }
    parts.join("-")
}

pub fn get_start_end_of(
    datetime: DateTime<Utc>,
    group: GroupByDate,
) -> (DateTime<Utc>, DateTime<Utc>) {
    let naive = datetime.naive_utc();

    let start_naive = match group {
        GroupByDate::Year => NaiveDate::from_ymd_opt(naive.year(), 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),

        GroupByDate::Month => NaiveDate::from_ymd_opt(naive.year(), naive.month(), 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),

        GroupByDate::Day => NaiveDate::from_ymd_opt(naive.year(), naive.month(), naive.day())
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),

        GroupByDate::Hour => NaiveDate::from_ymd_opt(naive.year(), naive.month(), naive.day())
            .unwrap()
            .and_hms_opt(naive.hour(), 0, 0)
            .unwrap(),
    };

    let end_naive = match group {
        GroupByDate::Year => NaiveDate::from_ymd_opt(naive.year(), 12, 31)
            .unwrap()
            .and_hms_opt(23, 59, 59)
            .unwrap(),

        GroupByDate::Month => {
            let (year, month) = (naive.year(), naive.month());
            let next_month = if month == 12 {
                NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
            } else {
                NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
            };
            let last_day = next_month - Duration::days(1);
            last_day.and_hms_opt(23, 59, 59).unwrap()
        }

        GroupByDate::Day => NaiveDate::from_ymd_opt(naive.year(), naive.month(), naive.day())
            .unwrap()
            .and_hms_opt(23, 59, 59)
            .unwrap(),

        GroupByDate::Hour => NaiveDate::from_ymd_opt(naive.year(), naive.month(), naive.day())
            .unwrap()
            .and_hms_opt(naive.hour(), 59, 59)
            .unwrap(),
    };

    (
        Utc.from_utc_datetime(&start_naive),
        Utc.from_utc_datetime(&end_naive),
    )
}
