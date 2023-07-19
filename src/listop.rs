#[derive(Clone, Copy)]
pub enum ListOp<'a> {
    Nav(crate::ListDir),
    New(&'a str, &'a str),
}
