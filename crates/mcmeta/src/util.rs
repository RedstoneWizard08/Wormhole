use serde::{de::DeserializeOwned, Deserialize, Deserializer};

pub fn unwrap_list<'de, D, T: DeserializeOwned>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Default)]
    struct Inner<T> {
        #[serde(rename = "$value")]
        data: Vec<T>,
    }

    Ok(Inner::<T>::deserialize(deserializer)?.data)
}
