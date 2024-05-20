// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

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
