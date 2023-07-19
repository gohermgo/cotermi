#[derive(Clone, Copy)]
pub enum ListDir {
    ExitPoint,
    Up,
    Down,
}

#[derive(Clone, Copy)]
pub enum ListOp<'a> {
    Nav(ListDir),
    New(&'a str, &'a str),
}
