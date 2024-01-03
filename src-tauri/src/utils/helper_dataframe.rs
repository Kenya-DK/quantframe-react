#[derive(Debug)]
pub enum ColumnType {
    Bool,
    F64,
    I64,
    I32,
    String,
}
pub enum ColumnValues {
    Bool(Vec<bool>),
    F64(Vec<f64>),
    I64(Vec<i64>),
    I32(Vec<i32>),
    String(Vec<String>),
}
impl ColumnValues {
    /// Converts the ColumnValues into a vector of Strings, if it is a String.
    /// Returns None if the ColumnValues is not a String.
    pub fn into_string(self) -> Option<Vec<String>> {
        match self {
            ColumnValues::String(values) => Some(values),
            _ => None,
        }
    }

    /// Converts the ColumnValues into a vector of bools, if it is a Bool.
    /// Returns None if the ColumnValues is not a Bool.
    pub fn into_bool(self) -> Option<Vec<bool>> {
        match self {
            ColumnValues::Bool(values) => Some(values),
            _ => None,
        }
    }

    /// Converts the ColumnValues into a vector of f64s, if it is a F64.
    /// Returns None if the ColumnValues is not a F64.
    pub fn into_f64(self) -> Option<Vec<f64>> {
        match self {
            ColumnValues::F64(values) => Some(values),
            _ => None,
        }
    }

    /// Converts the ColumnValues into a vector of i64s, if it is a I64.
    /// Returns None if the ColumnValues is not a I64.
    pub fn into_i64(self) -> Option<Vec<i64>> {
        match self {
            ColumnValues::I64(values) => Some(values),
            _ => None,
        }
    }

    /// Converts the ColumnValues into a vector of i32s, if it is a I32.
    /// Returns None if the ColumnValues is not a I32.
    pub fn into_i32(self) -> Option<Vec<i32>> {
        match self {
            ColumnValues::I32(values) => Some(values),
            _ => None,
        }
    }
}
pub enum ColumnValue {
    Bool(Option<bool>),
    F64(Option<f64>),
    I64(Option<i64>),
    I32(Option<i32>),
    String(Option<String>),
}
impl ColumnValue {
    /// Converts the ColumnValue into a String, if it is a String.
    /// Returns None if the ColumnValue is not a String.
    pub fn into_string(self) -> Option<String> {
        match self {
            ColumnValue::String(value) => value,
            _ => None,
        }
    }

    /// Converts the ColumnValue into a bool, if it is a Bool.
    /// Returns None if the ColumnValue is not a Bool.
    pub fn into_bool(self) -> Option<bool> {
        match self {
            ColumnValue::Bool(value) => value,
            _ => None,
        }
    }

    /// Converts the ColumnValue into a f64, if it is a F64.
    /// Returns None if the ColumnValue is not a F64.
    pub fn into_f64(self) -> Option<f64> {
        match self {
            ColumnValue::F64(value) => value,
            _ => None,
        }
    }

    /// Converts the ColumnValue into a i64, if it is a I64.
    /// Returns None if the ColumnValue is not a I64.
    pub fn into_i64(self) -> Option<i64> {
        match self {
            ColumnValue::I64(value) => value,
            _ => None,
        }
    }

    /// Converts the ColumnValue into a i32, if it is a I32.
    /// Returns None if the ColumnValue is not a I32.
    pub fn into_i32(self) -> Option<i32> {
        match self {
            ColumnValue::I32(value) => value,
            _ => None,
        }
    }
}
/// Extracts the values of a specified column from a DataFrame.
///
/// The function takes a DataFrame, an optional filter expression, the name of the column to extract, and the expected type of the column.
/// If a filter is provided, it is applied to the DataFrame before extracting the column.
/// The function returns a Result containing the extracted column values, or an AppError if an error occurs.
///
/// # Arguments
///
/// * `df` - The DataFrame to extract the column from.
/// * `filter` - An optional filter expression to apply to the DataFrame before extracting the column.
/// * `column` - The name of the column to extract.
/// * `col_type` - The expected type of the column.
///
/// # Returns
///
/// A Result containing a ColumnValues enum representing the extracted column values, or an AppError if an error occurs.
pub fn get_column_values(
    df: DataFrame,
    filter: Option<Expr>,
    column: &str,
    col_type: ColumnType,
) -> Result<ColumnValues, AppError> {
    let df: DataFrame = match filter {
        Some(filter) => df.lazy().filter(filter).collect().map_err(|e| {
            AppError::new(
                "Helper",
                eyre!(format!(
                    "Column: {:?} ColumnType: {:?} Error: {:?}",
                    column, col_type, e
                )),
            )
        })?,
        None => df,
    };

    let column_series = df.column(column).map_err(|e| {
        AppError::new(
            "Helper",
            eyre!(format!(
                "Column: {:?} ColumnType: {:?} Error: {:?}",
                column, col_type, e
            )),
        )
    })?;

    match col_type {
        ColumnType::Bool => {
            let values: Vec<bool> = column_series
                .bool()
                .map_err(|e| {
                    AppError::new(
                        "Helper",
                        eyre!(format!(
                            "Column: {:?} ColumnType: {:?} Error: {:?}",
                            column, col_type, e
                        )),
                    )
                })?
                .into_iter()
                .filter_map(|opt_val| opt_val)
                .collect();
            Ok(ColumnValues::Bool(values))
        }

        ColumnType::F64 => {
            let values: Vec<f64> = column_series
                .f64()
                .map_err(|e| {
                    AppError::new(
                        "Helper",
                        eyre!(format!(
                            "Column: {:?} ColumnType: {:?} Error: {:?}",
                            column, col_type, e
                        )),
                    )
                })?
                .into_iter()
                .filter_map(|opt_val| opt_val)
                .collect();
            Ok(ColumnValues::F64(values))
        }

        ColumnType::I64 => {
            let values: Vec<i64> = column_series
                .i64()
                .map_err(|e| {
                    AppError::new(
                        "Helper",
                        eyre!(format!(
                            "Column: {:?} ColumnType: {:?} Error: {:?}",
                            column, col_type, e
                        )),
                    )
                })?
                .into_iter()
                .filter_map(|opt_val| opt_val)
                .collect();
            Ok(ColumnValues::I64(values))
        }
        ColumnType::I32 => {
            let values: Vec<i32> = column_series
                .i32()
                .map_err(|e| {
                    AppError::new(
                        "Helper",
                        eyre!(format!(
                            "Column: {:?} ColumnType: {:?} Error: {:?}",
                            column, col_type, e
                        )),
                    )
                })?
                .into_iter()
                .filter_map(|opt_val| opt_val)
                .collect();
            Ok(ColumnValues::I32(values))
        }
        ColumnType::String => {
            let values = column_series
                .utf8()
                .map_err(|e| {
                    AppError::new(
                        "Helper",
                        eyre!(format!(
                            "Column: {:?} ColumnType: {:?} Error: {:?}",
                            column, col_type, e
                        )),
                    )
                })?
                .into_iter()
                .filter_map(|opt_name| opt_name.map(String::from))
                .collect::<Vec<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            Ok(ColumnValues::String(values))
        }
    }
}

/// Extracts the first value of a specified column from a DataFrame.
///
/// The function takes a DataFrame, an optional filter expression, the name of the column to extract, and the expected type of the column.
/// If a filter is provided, it is applied to the DataFrame before extracting the column.
/// The function returns a Result containing the first value of the extracted column, or an AppError if an error occurs.
///
/// # Arguments
///
/// * `df` - The DataFrame to extract the column from.
/// * `filter` - An optional filter expression to apply to the DataFrame before extracting the column.
/// * `column` - The name of the column to extract.
/// * `col_type` - The expected type of the column.
///
/// # Returns
///
/// A Result containing a ColumnValue enum representing the first value of the extracted column, or an AppError if an error occurs.
pub fn get_column_value(
    df: DataFrame,
    filter: Option<Expr>,
    column: &str,
    col_type: ColumnType,
) -> Result<ColumnValue, AppError> {
    match get_column_values(df, filter, column, col_type)? {
        ColumnValues::Bool(bool_values) => {
            let value = bool_values.get(0).cloned();
            Ok(ColumnValue::Bool(value))
        }
        ColumnValues::F64(f64_values) => {
            let value = f64_values.get(0).cloned();
            Ok(ColumnValue::F64(value))
        }
        ColumnValues::I64(i64_values) => {
            let value = i64_values.get(0).cloned();
            Ok(ColumnValue::I64(value))
        }
        ColumnValues::I32(i32_values) => {
            let value = i32_values.get(0).cloned();
            Ok(ColumnValue::I32(value))
        }
        ColumnValues::String(string_values) => {
            let value = string_values.get(0).cloned();
            Ok(ColumnValue::String(value))
        }
    }
}

/// Sorts a DataFrame based on a specified column.
///
/// The function takes a DataFrame, an optional filter expression, the name of the column to sort by, and a boolean indicating whether to sort in ascending order.
/// If a filter is provided, it is applied to the DataFrame before sorting.
/// The function returns a Result containing the sorted DataFrame, or an AppError if an error occurs.
///
/// # Arguments
///
/// * `df` - The DataFrame to sort.
/// * `filter` - An optional filter expression to apply to the DataFrame before sorting.
/// * `column` - The name of the column to sort by.
/// * `ascending` - A boolean indicating whether to sort in ascending order.
///
/// # Returns
///
/// A Result containing the sorted DataFrame, or an AppError if an error occurs.
pub fn sort_dataframe(
    df: DataFrame,
    filter: Option<Expr>,
    column: &str,
    ascending: bool,
) -> Result<DataFrame, AppError> {
    let df = match filter {
        Some(filter) => df
            .lazy()
            .filter(filter)
            .collect()
            .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?,
        None => df,
    };

    let df = df
        .clone()
        .lazy()
        .sort(
            column,
            SortOptions {
                descending: ascending,
                nulls_last: false,
                multithreaded: false,
            },
        )
        .collect()
        .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?;
    Ok(df)
}

/// Filters a DataFrame based on a specified expression and extracts selected columns.
///
/// The function takes a DataFrame, an optional filter expression, and a vector of column names to extract.
/// If a filter is provided, it is applied to the DataFrame before extracting the columns.
/// The function returns a Result containing the filtered DataFrame with only the selected columns, or an AppError if an error occurs.
///
/// # Arguments
///
/// * `df` - The DataFrame to filter and extract columns from.
/// * `filter` - An optional filter expression to apply to the DataFrame before extracting the columns.
/// * `select_cols` - A vector of column names to extract.
///
/// # Returns
///
/// A Result containing the filtered DataFrame with only the selected columns, or an AppError if an error occurs.
pub fn filter_and_extract(
    df: DataFrame,
    filter: Option<Expr>,
    select_cols: Vec<&str>,
) -> Result<DataFrame, AppError> {
    let selected_columns: Vec<_> = select_cols.into_iter().map(col).collect();

    let df = match filter {
        Some(filter) => df
            .lazy()
            .filter(filter)
            .collect()
            .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?,
        None => df,
    };

    let df_select = df.lazy().select(&selected_columns).collect();
    match df_select {
        Ok(df_select) => Ok(df_select),
        Err(e) => Err(AppError::new("Helper", eyre!(e.to_string()))),
    }
}

/// Merges multiple DataFrames into a single DataFrame.
///
/// The function takes a vector of DataFrames to merge. It stacks the series from all frames vertically for each column.
/// The function returns a Result containing the merged DataFrame, or an AppError if an error occurs.
///
/// # Arguments
///
/// * `frames` - A vector of DataFrames to merge.
///
/// # Returns
///
/// A Result containing the merged DataFrame, or an AppError if an error occurs.
pub fn merge_dataframes(frames: Vec<DataFrame>) -> Result<DataFrame, AppError> {
    // Check if there are any frames to merge
    if frames.is_empty() {
        return Err(AppError::new("Helper", eyre!("No frames to merge")));
    }

    // Get the column names from the first frame
    let column_names: Vec<&str> = frames[0].get_column_names();

    // For each column name, stack the series from all frames vertically
    let mut combined_series: Vec<Series> = Vec::new();

    for &col_name in &column_names {
        let first_series = frames[0]
            .column(col_name)
            .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?
            .clone();
        let mut stacked_series = first_series;

        for frame in frames.iter().skip(1) {
            let series = frame
                .column(col_name)
                .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?
                .clone();
            stacked_series = stacked_series
                .append(&series)
                .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?
                .clone();
        }

        combined_series.push(stacked_series);
    }
    // Construct a DataFrame from the merged data
    Ok(DataFrame::new(combined_series)
        .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?)
}
