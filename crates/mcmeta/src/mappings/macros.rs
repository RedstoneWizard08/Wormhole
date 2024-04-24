#[macro_export]
macro_rules! eq_or_err {
    ($($var: expr)+; == $($expected: expr)+; !=> $($err: expr)+) => {
        {
            let val = $($var)+;
            let exp = $($expected)+;

            if val != exp {
                return Err($crate::anyhow::anyhow!($($err)+, val = val, exp = exp))
            }
        }
    }
}
