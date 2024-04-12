pub extern crate colored;
pub extern crate tracing;

pub mod logging;
pub mod middleware;

use middleware::FILTERS;

pub use middleware::logging_middleware;

pub fn add_route_filter(filter: &'static str) {
    FILTERS.lock().unwrap().push(filter);
}
