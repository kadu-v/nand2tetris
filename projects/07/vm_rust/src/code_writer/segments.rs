//! segmemts for Hack VM

#[derive(Debug, Eq, PartialEq)]
pub enum Segments {
    Argument,
    Local,
    Static,
    This,
    That,
    Pointer,
    Temp,
}
