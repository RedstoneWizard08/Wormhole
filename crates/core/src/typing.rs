use specta::TypeMap;

pub fn merge_type_maps(maps: Vec<TypeMap>) -> TypeMap {
    let mut map = TypeMap::default();

    for ty_map in maps {
        for (id, ty) in ty_map.iter() {
            map.insert(id, ty.clone());
        }
    }

    map
}
