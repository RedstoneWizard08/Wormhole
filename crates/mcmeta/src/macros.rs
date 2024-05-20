// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

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
