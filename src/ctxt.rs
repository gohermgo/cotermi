use crate::{list, signal};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Ctxt {
    Default,
    List,
}
impl Ctxt {
    pub fn processs_input(&self, key: &KeyEvent) -> Option<signal::Signal> {
        match self {
            &Default => match key.code {
                KeyCode::Char('Q') | KeyCode::Char('q') => Some(signal::Signal::Quit),
                KeyCode::Char('l') => Some(signal::Signal::ChCtxt(Ctxt::List)),
                KeyCode::Left => Some(signal::Signal::List(list::ListOp::Nav(
                    list::ListDir::ExitPoint,
                ))),
                KeyCode::Up => Some(signal::Signal::List(list::ListOp::Nav(list::ListDir::Prev))),
                KeyCode::Down => Some(signal::Signal::List(list::ListOp::Nav(list::ListDir::Down))),
                _ => None,
            },
            &List => match key.code {},
        }
    }
}
