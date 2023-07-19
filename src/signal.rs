#[derive(Clone, Copy)]
pub enum Signal<'a> {
    Quit,
    ChCtxt(crate::ctxt::Ctxt),
    List(crate::list::ListOp<'a>),
}
