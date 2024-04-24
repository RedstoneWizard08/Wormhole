use std::collections::HashMap;

pub fn parse_manifest(data: impl AsRef<str>) -> HashMap<String, String> {
    let lines = data.as_ref().lines().collect::<Vec<_>>();
    let mut map = HashMap::new();

    for line in lines {
        let mut spl = line.split(":").map(|v| v.to_string()).collect::<Vec<_>>();
        let key = spl.remove(0);
        let val = spl.join(":");

        map.insert(key.trim().to_string(), val.trim().to_string());
    }

    map
}
