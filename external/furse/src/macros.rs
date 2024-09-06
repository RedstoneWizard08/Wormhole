#[macro_export]
macro_rules! optional_insert {
    ($map: ident += $key: expr => $str: ident.$str_key: ident) => {
        if let Some(val) = $str.$str_key {
            $map.insert($key, val);
        }
    };

    ($map: ident += $key: expr => $str: ident.$str_key: ident into !String) => {
        if let Some(val) = $str.$str_key {
            let out = val.to_string();

            $map.insert($key, out);
        }
    };

    ($map: ident += $key: expr => $str: ident.$str_key: ident as $as: ty => into !String) => {
        if let Some(val) = $str.$str_key {
            let out = (val as $as).to_string();

            $map.insert($key, out);
        }
    };

    ($map: ident += $key: expr => $str: ident.$str_key: ident into $as: ty) => {
        if let Some(val) = $str.$str_key {
            let out: $as = val.into();

            $map.insert($key, out);
        }
    };

    ($map: ident += $key: expr => $str: ident.$str_key: ident [vec] into $as: ty) => {
        if let Some(val) = $str.$str_key {
            let mut out: Vec<$as> = Vec::new();

            for item in val {
                out.push(item.into());
            }

            $map.insert($key, out);
        }
    };

    ($map: ident += $key: expr => $str: ident.$str_key: ident [vec + $sep: expr] into $as: ty) => {
        if let Some(val) = $str.$str_key {
            let mut out: Vec<$as> = Vec::new();

            for item in val {
                out.push(item.into());
            }

            $map.insert($key, out.join($sep));
        }
    };

    ($map: ident += $key: expr => $str: ident.$str_key: ident [vec + $sep: expr] into !String) => {
        if let Some(val) = $str.$str_key {
            let mut out: Vec<String> = Vec::new();

            for item in val {
                out.push(item.to_string());
            }

            $map.insert($key, out.join($sep));
        }
    };

    ($map: ident += $key: expr => $str: ident.$str_key: ident [vec + $sep: expr] as $as: ty => into !String) => {
        if let Some(val) = $str.$str_key {
            let mut out: Vec<String> = Vec::new();

            for item in val {
                out.push((item as $as).to_string());
            }

            $map.insert($key, out.join($sep));
        }
    };

    ($map: ident += $key: expr => $str: ident.$str_key: ident [vec + $sep: expr]) => {
        if let Some(val) = $str.$str_key {
            $map.insert($key, val.join($sep));
        }
    };
}
