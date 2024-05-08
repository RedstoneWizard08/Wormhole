#[macro_export]
macro_rules! cmd_string_fix {
    ($var: ident, $opts: ident, $name: ident -> p:$field: ident) => {
        $var = $var.replace(&format!("${{{}}}", stringify!($name)), $field.to_str().unwrap());
    };

    ($var: ident, $opts: ident, $name: ident -> s:$str: expr) => {
        $var = $var.replace(&format!("${{{}}}", stringify!($name)), $str);
    };

    ($var: ident, $opts: ident, $name: ident -> a:$field: ident) => {
        if let Some($field) = &$opts.$field {
            $var = $var.replace(&format!("${{{}}}", stringify!($name)), $field);
        }
    };

    ($var: ident, $opts: ident, $name: ident -> o:$($value: tt)+) => {
        $var = $var.replace(&format!("${{{}}}", stringify!($name)), &$opts.$($value)+.to_string());
    };

    ($var: ident, $opts: ident, $name: ident -> $($value: tt)+) => {
        $var = $var.replace(&format!("${{{}}}", stringify!($name)), &$($value)+.to_string());
    };
}
