use crate::Error;
use serde::{ser::SerializeMap, Serialize, Serializer};
use std::format;

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;

        map.serialize_entry("cause", &format!("{}", self))?;

        map.end()
    }
}
