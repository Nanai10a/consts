#[macro_export]
macro_rules! consts {
    () => {};
    ($i:ident : $l:literal; $( $t:tt )*) => {
        pub const $i: &'static str = $l;
        crate::consts! { $( $t )* }
    };
    ($m:ident { $( $t1:tt )* } $( $t2:tt )*) => {
        pub mod $m {
            crate::consts! { $( $t1 )* }
        }
        crate::consts! { $( $t2 )* }
    };
}

mod test;
