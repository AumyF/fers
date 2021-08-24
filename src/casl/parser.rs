#[derive(Clone)]
struct Position {
    line: usize,
    column: usize,
}

#[derive(Clone)]
struct ParserState<'a> {
    lines: &'a [&'a str],
    position: Position,
}

struct ParserError {
    label: String,
    message: String,
    position: Position,
}
type ParseResult<T> = Result<T, ParserError>;

/// パーサコンビネータ
struct Parser {
    parse_fn: Box<dyn Fn(ParserState) -> Result<(), ()>>,
    label: String,
}

impl Position {
    fn increment_line(&self) -> Position {
        Position {
            line: self.line + 1,
            ..*self
        }
    }
    fn increment_column(&self) -> Position {
        Position {
            column: self.column + 1,
            line: 0,
        }
    }
}

impl ParserState<'_> {
    /// 入力文字列の終端に来ているかどうか調べる
    fn is_at_end_of_input(&self) -> bool {
        self.position.line >= self.lines.len()
    }
    /// 現在の行を取得する．行の終端に到達した場合 `None`
    fn get_current_line(&self) -> Option<&&str> {
        self.lines.get(self.position.line)
    }
    /// 文字を取得して1文字進める
    fn get_next_char(&self) -> (ParserState, Option<char>) {
        let position_line = self.position.line;
        if self.is_at_end_of_input() {
            (self.clone(), None)
        } else {
            match self.get_current_line() {
                Some(current_line) => {
                    let position = self.position.increment_column();
                    let ch = current_line.chars().nth(0).unwrap();
                    (ParserState { position, ..*self }, Some(ch))
                }
                None => {
                    let position = self.position.increment_line();
                    (ParserState { position, ..*self }, Some('\n'))
                }
            }
        }
    }
}

// #[cfg(test)]
// mod test {
//     #[test]
//     fn g() {
//         ae
//     }
// }
fn f() {
    let a = Some(());
    let b = Some(());
    let c = Some(());
}
