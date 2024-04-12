#[macro_export]
macro_rules! midlog_log {
    ($prefix: expr, $route: expr, $status: expr, $time: expr) => {
        let time = $crate::colored::Colorize::bright_blue(format!("({} ms)", $time).as_str());

        $crate::tracing::event!(
            target: "midlog::logging",
            $crate::tracing::Level::INFO,
            "{} {} {} {}",
            $crate::colored::Colorize::cyan($prefix),
            $crate::colored::Colorize::magenta($route),
            $status,
            time,
        );
    };
}

#[macro_export]
macro_rules! log_get {
    ($route: expr, $status: expr, $time: expr) => {
        $crate::logging::midlog_log!("GET", $route, $status, $time)
    };
}

#[macro_export]
macro_rules! log_post {
    ($route: expr, $status: expr, $time: expr) => {
        $crate::logging::midlog_log!("POST", $route, $status, $time)
    };
}

#[macro_export]
macro_rules! log_put {
    ($route: expr, $status: expr, $time: expr) => {
        $crate::logging::midlog_log!("PUT", $route, $status, $time)
    };
}

#[macro_export]
macro_rules! log_delete {
    ($route: expr, $status: expr, $time: expr) => {
        $crate::logging::midlog_log!("DELETE", $route, $status, $time)
    };
}

#[macro_export]
macro_rules! log_patch {
    ($route: expr, $status: expr, $time: expr) => {
        $crate::logging::midlog_log!("PATCH", $route, $status, $time)
    };
}
