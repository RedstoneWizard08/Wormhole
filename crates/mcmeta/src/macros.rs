#[macro_export]
macro_rules! cmd_string_fix {
    ($var: ident, $opts: ident, $name: ident -> $field: ident) => {
        $var = $var.replace(stringify!($name), &$opts.$field.to_string());
    };

    ($var: ident, $opts: ident, $name: ident -> p:$field: ident) => {
        $var = $var.replace(stringify!($name), $opts.$field.to_str().unwrap());
    };

    ($var: ident, $opts: ident, $name: ident -> s:$str: expr) => {
        $var = $var.replace(&format!("${{{}}}", stringify!($name)), $str);
    };

    ($var: ident, $opts: ident, $name: ident -> o:$field: ident) => {
        if let Some($field) = &$opts.$field {
            $var = $var.replace(&format!("${{{}}}", stringify!($name)), $field);
        }
    };
}
