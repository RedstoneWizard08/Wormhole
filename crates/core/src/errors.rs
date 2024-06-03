pub trait Stringify<T> {
    fn stringify(self) -> std::result::Result<T, String>;
}

impl<T, E: std::fmt::Display> Stringify<T> for std::result::Result<T, E> {
    fn stringify(self) -> std::result::Result<T, String> {
        self.map_err(|err| format!("{}", err))
    }
}

impl<T> Stringify<T> for Option<T> {
    fn stringify(self) -> std::result::Result<T, String> {
        self.ok_or("Tried to unwrap a None value!".into())
    }
}
