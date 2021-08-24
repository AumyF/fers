use crate::list;

use super::ast;
use super::list;
use std::process::Output;
use std::str;

// fn parse(string: &str) -> ast::Source {
//     string
// }

pub trait ParseResult {
    fn get(&self) -> String;
}

// impl<T: ToString> TS for Option<T> {
//     fn get(&self) -> String {
//         self.map(|t| t.to_string()).unwrap_or("".to_string())
//     }
// }

// impl<T: ParseResult> ParseResult for T {
//     fn get(&self) -> String {
//         self.to_string()
//     }
// }

#[derive(Debug, PartialEq, Eq)]
/// 1文字
struct Charactor(pub char);
impl Charactor {
    /// charを受け取る関数をCharactor対応にする
    fn lift<F>(f: impl FnOnce(char) -> F) -> impl FnOnce(Charactor) -> F {
        move |Charactor(char): Charactor| f(char)
    }
}
impl ToString for Charactor {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

trait ParseFn<O> {
    fn parse(&self, ps: &mut ParserState) -> O;
}

impl<F: Fn(&mut ParserState) -> O, O> ParseFn<O> for F {
    fn parse(&self, ps: &mut ParserState) -> O {
        self(ps)
    }
}

impl ParseFn<String> for list::Nil {
    fn parse(&self, ps: &mut ParserState) -> String {
        "".to_string()
    }
}

impl<HR, TR, H, T> ParseFn<(HR, TR)> for list::Cons<H, T>
where
    HR: ParseResult,
    TR: ParseResult,
    H: ParseFn<HR>,
    T: ParseFn<TR>,
{
    fn parse(&self, ps: &mut ParserState) -> (HR, TR) {
        let list::Cons { head, tail } = self;
        (head.parse(ps), tail.parse(ps))
    }
}

// trait Parser<T: ParseResult>: Fn(&ParserState) -> T {}

// impl<HR, TR, H, T> FnOnce<(&ParserState,)> for list::Cons<H, T>
// where
//     HR: ParseResult,
//     TR: ParseResult,
//     H: Fn(&mut ParserState) -> HR,
//     T: Fn(&mut ParserState) -> TR,
// {
//     type Output = TR;
// }

// impl ParseFn for list::Nil {
//     fn parse(&self, _: &mut ParserState) -> impl ParseResult {
//         "".to_string()
//     }
// }

// impl<H, T> list::Cons<H, T>
// where
//     T: ParseFn,
//     H: ParseFn,
// {
//     fn parse(&self, ps: &mut ParserState) -> impl ParseResult {
//         let Self { head, tail } = self;
//         format!("{}{}", head.parse::<T>(ps), tail.parse(ps))
//     }
// }
// impl<HR, TR, H, T, Q> list::Cons<H, list::Cons<T, Q>>
// where
//     HR: ParseResult,
//     TR: ParseResult,
//     H: Fn(&mut ParserState) -> HR,
//     T: Fn(&mut ParserState) -> TR,
// {
//     fn call(&self, p: &mut ParserState) -> impl ParseResult {
//         let Self { head, tail } = self;
//         let list::Cons { head: middle, tail } = tail;
//         format!("{}{}", head(p).to_string(), middle(p).to_string())
//     }
// }

#[derive(Debug)]
pub struct ParserState {
    position: usize,
    string: String,
}

impl ParserState {
    pub fn new(s: &str) -> ParserState {
        ParserState {
            position: 0,
            string: s.to_string(),
        }
    }
    pub fn peek(&self) -> Option<char> {
        self.string
            .get(self.position..self.position + 1)?
            .chars()
            .nth(0)
    }
    pub fn next(&mut self) {
        self.position += 1;
    }

    /// 任意の1文字にマッチする。
    pub fn any_char(&mut self) -> Option<char> {
        self.satisfy(|_| true)
    }

    /// 条件でマッチする。
    pub fn satisfy<F: FnOnce(char) -> bool>(&mut self, f: F) -> Option<char> {
        let chara = self.peek()?;
        if !f(chara) {
            None
        } else {
            self.next();
            Some(chara)
        }
    }

    // pub fn sequence<'a>(
    //     parsers: Box<[impl Fn(&mut Parser<'a>) -> Option<char>]>,
    // ) -> impl Fn(&'a mut Parser<'a>) -> Option<String> {
    //     |p| {
    //         parsers.iter().fold(Some("".to_string()), |string, f| {
    //             Some(format!("{}{}", string?, f(p)?))
    //         })
    //     }
    // }

    pub fn seq<F>(
        p: &'static Vec<&impl Fn(&mut ParserState) -> Option<char>>,
    ) -> impl Fn(&mut ParserState) -> Option<String>
    where
        F: Fn(&mut ParserState) -> Option<char>,
    {
        move |psr: &mut ParserState| {
            p.iter().fold(Some("".to_string()), |string, f| {
                Some(format!("{}{}", string?, f(psr)?))
            })
        }
    }

    /// 指定した1文字にマッチする。
    pub fn char(&mut self, ch: char) -> Option<char> {
        self.satisfy(|c| c == ch)
    }
    /// いかなる文字にもマッチしない。
    pub fn never(&mut self) -> Option<char> {
        self.satisfy(|_| false)
    }
    pub fn digit(&mut self) -> Option<char> {
        self.satisfy(char::is_numeric)
    }
    pub fn upper(&mut self) -> Option<char> {
        self.satisfy(char::is_uppercase)
    }
    pub fn lower(&mut self) -> Option<char> {
        self.satisfy(char::is_lowercase)
    }
    pub fn ascii_alphabet(&mut self) -> Option<char> {
        self.satisfy(|c| c.is_ascii_alphabetic())
    }
    pub fn ascii_alphanumeric(&mut self) -> Option<char> {
        self.satisfy(|c| c.is_ascii_alphanumeric())
    }

    pub fn join<F, G>(
        f: impl Fn(&mut ParserState) -> Option<F>,
        g: impl Fn(&mut ParserState) -> Option<G>,
    ) -> impl Fn(&mut ParserState) -> Option<list::Cons<F, list::Cons<G, list::Nil>>> {
        move |p| Some(list!(f(p)?, g(p)?))
    }
}

// macro_rules! sequence {
//     ( $head:expr, $next:expr) => { ParserState::join($head, $next) };

//     ( $head:expr, $next:expr, $($cons:expr), +) => {
//         sequence!(ParserState::join($head, $next), $($cons), +)
//     };
// }

#[cfg(test)]
mod test {
    use crate::casl::_parser::ParseFn;

    use super::Charactor;
    use super::ParseResult;
    use super::ParserState;
    use itertools::concat;
    #[test]
    fn any_char() {
        assert_eq!(ParserState::new("Hello, world").any_char(), Some('H'));
        assert_eq!(ParserState::new("おはよう").any_char(), Some('お'));
        assert_eq!(ParserState::new("🦀").any_char(), Some('🦀'));
    }

    #[test]
    fn satisfy() {
        assert_eq!(ParserState::new("123").satisfy(char::is_numeric), Some('1'));
        assert_eq!(ParserState::new("abc").satisfy(char::is_numeric), None);
    }

    #[test]
    fn char() {
        assert_eq!(ParserState::new("hello").char('h'), Some('h'));
        assert_eq!(ParserState::new("hello").char('お'), None);
        assert_eq!(ParserState::new("🦀🦐").char('🦀'), Some('🦀'));
    }

    // #[test]
    // fn join() {
    //     let h = ParserState::join(ParserState::ascii_alphabet, ParserState::digit);
    //     assert_eq!(ParserState::new("ho").ho(), Some(['h', 'o']))
    // }

    impl ParserState {
        // fn ho(&mut self) -> Option<[char; 2]> {
        //     use super::list;
        //     list!(ParserState::ascii_alphabet, ParserState::digit)
        // }
        fn a_d_d(&mut self) -> Option<String> {
            let c1 = self.ascii_alphabet()?;
            let c2 = self.digit()?;
            let c3 = self.digit()?;
            Some(format!("{}{}{}", c1, c2, c3))
        }
        fn add(&mut self) -> Option<String> {
            use super::list;
            let l = list!(
                ParserState::ascii_alphabet,
                ParserState::digit,
                ParserState::digit
            );
            let l = list::Cons {
                head: ParserState::ascii_alphabet,
                tail: list::Nil,
            };
            l.parse()
        }
    }

    #[test]
    fn a_d_d() {
        assert_eq!(ParserState::new("a21").a_d_d(), Some("a21".to_string()));
        assert_eq!(ParserState::new("2ae").a_d_d(), None);
        assert_eq!(ParserState::new("ae2").a_d_d(), None);
    }
}
