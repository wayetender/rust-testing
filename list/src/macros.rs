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
