#[macro_export]
macro_rules! log_info {
    ($component:expr, $($arg:tt)*) => {
        $crate::info($component, format!($($arg)*), $crate::LoggerOptions::default())
    };
}

#[macro_export]
macro_rules! log_info_opt {
    ($component:expr, $opts:expr, $($arg:tt)*) => {
        $crate::info($component, format!($($arg)*), $opts)
    };
}

#[macro_export]
macro_rules! log_error {
    ($component:expr, $($arg:tt)*) => {
        $crate::error($component, format!($($arg)*), $crate::LoggerOptions::default())
    };
}

#[macro_export]
macro_rules! log_error_opt {
    ($component:expr, $opts:expr, $($arg:tt)*) => {
        $crate::error($component, format!($($arg)*), $opts)
    };
}

#[macro_export]
macro_rules! log_debug {
    ($component:expr, $($arg:tt)*) => {
        $crate::debug($component, format!($($arg)*), $crate::LoggerOptions::default())
    };
}

#[macro_export]
macro_rules! log_debug_opt {
    ($component:expr, $opts:expr, $($arg:tt)*) => {
        $crate::debug($component, format!($($arg)*), $opts)
    };
}

#[macro_export]
macro_rules! log_warn {
    ($component:expr, $($arg:tt)*) => {
        $crate::warning($component, format!($($arg)*), $crate::LoggerOptions::default())
    };
}

#[macro_export]
macro_rules! log_warn_opt {
    ($component:expr, $opts:expr, $($arg:tt)*) => {
        $crate::warning($component, format!($($arg)*), $opts)
    };
}

#[macro_export]
macro_rules! log_critical {
    ($component:expr, $($arg:tt)*) => {
        $crate::critical($component, format!($($arg)*), $crate::LoggerOptions::default())
    };
}

#[macro_export]
macro_rules! log_critical_opt {
    ($component:expr, $opts:expr, $($arg:tt)*) => {
        $crate::critical($component, format!($($arg)*), $opts)
    };
}

// Zip logging macros
#[macro_export]
macro_rules! zip_log_info {
    ($zip_logger:expr, $component:expr, $($arg:tt)*) => {
        $zip_logger.add_log("INFO", $component, format!($($arg)*)).ok();
    };
}

#[macro_export]
macro_rules! zip_log_error {
    ($zip_logger:expr, $component:expr, $($arg:tt)*) => {
        $zip_logger.add_log("ERROR", $component, format!($($arg)*)).ok();
    };
}

#[macro_export]
macro_rules! zip_log_warn {
    ($zip_logger:expr, $component:expr, $($arg:tt)*) => {
        $zip_logger.add_log("WARN", $component, format!($($arg)*)).ok();
    };
}

#[macro_export]
macro_rules! zip_log_debug {
    ($zip_logger:expr, $component:expr, $($arg:tt)*) => {
        $zip_logger.add_log("DEBUG", $component, format!($($arg)*)).ok();
    };
}

#[macro_export]
macro_rules! zip_log_critical {
    ($zip_logger:expr, $component:expr, $($arg:tt)*) => {
        $zip_logger.add_log("CRITICAL", $component, format!($($arg)*)).ok();
    };
}

/// Macro to clear logs older than specified number of days
#[macro_export]
macro_rules! clear_logs {
    ($days:expr) => {
        $crate::clear_logs($days)
    };
}

#[macro_export]
macro_rules! get_location {
    () => {{
        let file = file!();
        let line = line!();
        let col = column!();
        format!("{}:{}:{}", file, line, col)
    }};
}
