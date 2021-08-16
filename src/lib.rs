#[macro_export]
macro_rules! consts {
    () => {};
    ($i:ident : $l:literal; $( $t:tt )*) => {
        pub const $i: &'static str = $l;
        consts! { $( $t )* }
    };
    ($m:ident { $( $t1:tt )* } $( $t2:tt )*) => {
        pub mod $m {
            consts! { $( $t1 )* }
        }
        consts! { $( $t2 )* }
    };
}

mod test;
