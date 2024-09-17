#[macro_export]
macro_rules! check_bytes {
    ($var: ident == $expected: expr) => {
        if &$var[..] != $expected {
            return Err($crate::anyhow::anyhow!(
                "Invalid bytes! Expected: {:?}; got: {:?}",
                $expected,
                $var
            ));
        }
    };

    (#read($n: expr) $var: ident == $expected: expr) => {
        let mut tmp_ = [0u8; $n];

        std::io::Read::read_exact(&mut $var, &mut tmp_)?;

        $crate::check_bytes!(tmp_ == $expected);
    };
}
