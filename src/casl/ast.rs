/// ソースコード
pub struct Source<'a>(Vec<Line<'a>>);

pub enum Line<'a> {
    OperationLine(OperationLine<'a>),
    CommentLine(CommentLine<'a>),
}

pub struct CommentLine<'a> {
    comment: &'a str,
}

pub struct Label<'a>(&'a str);
pub enum Opecode {}
pub enum Operand {}

pub enum OperationLine<'a> {
    OperandLine(OperandLine<'a>),
    NoOperandLine(NoOperandLine<'a>),
}

pub struct OperandLine<'a> {
    label: Option<Label<'a>>,
    opecode: Opecode,
    operand: Operand,
}

pub struct NoOperandLine<'a> {
    label: Option<Label<'a>>,
    opecode: Opecode,
}
