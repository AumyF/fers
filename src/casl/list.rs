pub struct Nil;
pub struct Cons<H, T> {
    pub head: H,
    pub tail: T,
}

impl ToString for Nil {
    fn to_string(&self) -> String {
        "".to_string()
    }
}

impl<H: ToString, T: ToString> ToString for Cons<H, T> {
    fn to_string(&self) -> String {
        format!("{}{}", self.head.to_string(), self.tail.to_string())
    }
}

impl<H, T> Cons<H, T> {
    fn fold() {}
}

#[macro_export]
macro_rules! list {
    ($tail:expr) => {
       crate::casl::list:: Cons {
            head: $tail,
            tail: crate::casl::list:: Nil,
        }
    };
    ( $head:expr, $( $cons:expr ), + ) => {
        crate::casl::list:: Cons {
            head: $head,
            tail: list!($($cons), +),
        }
    };
}
