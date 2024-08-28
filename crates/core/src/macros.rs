#[macro_export]
macro_rules! type_map {
    { $($ty: ident),* $(,)? } => {
        #[allow(unused_mut, unused_imports)]
        pub fn __specta_typemap() -> $crate::specta::TypeMap {
            use $crate::specta::NamedType;

            let mut map = $crate::specta::TypeMap::default();

            $(
                let ty = $ty::named_data_type(&mut map, &[]);
                map.insert($ty::sid(), ty);
            )*

            map
        }
    };
}
