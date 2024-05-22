use std::{future::Future, pin::Pin};

/// A progress callback.
///
/// Arguments:
/// - The total number of iterations. For a request or
///   file operation, this will be the total size of
///   the file, in bytes.
/// - The current processed iterations count. For a
///   request or file operation, this will be how many
///   bytes have been received/processed/written.
/// - The name of the file or operation being done.
pub type ProgressCallback = Box<
    dyn (Fn(u64, u64, String) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>) + Send + Sync,
>;
