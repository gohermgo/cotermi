pub mod list {
    #[derive(Clone, Copy)]
    pub enum Location {
        ExitPoint,
        Up,
        Down,
    }
    #[derive(Clone, Copy)]
    pub enum Operation<'a> {
        Go(Location),
        New(&'a str, &'a str),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Context {
    Default,
    List,
}
pub mod context {
    use super::{Action, Context};
    use crossterm::event::{KeyCode, KeyEvent};

    impl Context {
        pub fn processs_input(&self, key: &KeyEvent) -> Option<Action> {
            use super::list::*;
            match self {
                &Default => match key.code {
                    KeyCode::Char('Q') | KeyCode::Char('q') => Some(Action::Quit),
                    KeyCode::Char('l') => Some(Action::ChangeContext(Context::List)),
                    KeyCode::Left => Some(Action::ListSignal(Operation::Go(Location::ExitPoint))),
                    KeyCode::Up => Some(Action::ListSignal(Operation::Go(Location::Up))),
                    KeyCode::Down => Some(Action::ListSignal(Operation::Go(Location::Down))),
                    _ => None,
                },
                &List => match key.code {
                    KeyCode::Char('Q') => Some(Action::Quit),
                    KeyCode::Char('q') | KeyCode::Right | KeyCode::Esc => {
                        Some(Action::ChangeContext(Context::Default))
                    }
                    KeyCode::Up => Some(Action::ListSignal(Operation::Go(Location::Up))),
                    KeyCode::Down => Some(Action::ListSignal(Operation::Go(Location::Down))),
                    KeyCode::Char('N') | KeyCode::Char('n') => {
                        let new_title = "title";
                        let new_desc = "desc";
                        Some(Action::ListSignal(Operation::New(new_title, new_desc)))
                    }
                    _ => None,
                    _ => todo!(),
                },
            }
        }
    }
}
pub struct Signal<'a> {
    stype: SType,
    action: Action<'a>,
}
pub enum SType {
    TopLevel,
    Contextual,
}
pub enum Action<'a> {
    Quit,
    ChangeContext(Context),
    ListSignal(list::Operation<'a>),
}
pub mod signal {
    use super::Action;
    trait Contextual {}
    // impl Contextual for super::Action::
    // trait TopLevel {
    // fn process(&self) -> Fn();
    // }
    impl<'a> Action<'a> {
        fn perform(&self) {
            match self {
                Action::Quit => {}
                Action::ChangeContext(target_context) => {}
                Action::ListSignal(operation) => {}
            }
        }
    }
}
