#[derive(Clone, Debug, PartialEq)]
pub struct LineStart {
    pub(crate) offset: usize,
    pub(crate) line: i32,
}