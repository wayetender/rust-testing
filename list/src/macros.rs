
#[macro_export]
macro_rules! replace {
    ($x:expr, $y:expr) => {
        {
            use std::mem;
            let a = $x;
            let temp = a.take();
            let nnew = ($y)(temp);
            mem::replace(a, nnew);
        }
    };
}

#[macro_export]
macro_rules! option_satisfies {
    ($v:ident, $p:expr) => (
        $v.is_some() && (($p)($v.as_mut().unwrap()))
    )
}

