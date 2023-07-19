pub mod ctxt;
pub mod list;
pub mod signal;
pub mod utils;
use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

struct StatefulList<I> {
    state: ListState,
    exit_point: Option<usize>,
    list: Vec<I>,
}

impl<I> StatefulList<I> {
    fn with_items(items: Vec<I>) -> Self {
        Self {
            state: ListState::default(),
            exit_point: None,
            list: items,
        }
    }
    fn next(&mut self) -> io::Result<()> {
        let next_index = match self.state.selected() {
            Some(current_index) => {
                if current_index >= self.list.len() - 1 {
                    0
                } else {
                    current_index + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(next_index));
        Ok(())
    }
    fn prev(&mut self) -> io::Result<()> {
        let prev_index = match self.state.selected() {
            Some(current_index) => {
                if current_index == 0 {
                    self.list.len() - 1
                } else {
                    current_index - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(prev_index));
        Ok(())
    }
    fn deselect(&mut self) -> io::Result<()> {
        self.exit_point = self.state.selected();
        self.state.select(None);
        Ok(())
    }
    fn reselect(&mut self) -> io::Result<()> {
        let entry_point = match self.exit_point {
            Some(exit_point) => {
                if exit_point >= self.list.len() {
                    0
                } else {
                    exit_point
                }
            }
            None => 0,
        };
        self.state.select(Some(entry_point));
        Ok(())
    }
    fn reselect_next(&mut self) -> io::Result<()> {
        self.reselect()?;
        self.next()
    }
    fn reselect_prev(&mut self) -> io::Result<()> {
        self.reselect()?;
        self.prev()
    }
}
// #[derive(Clone, Copy)]
// pub enum ListDir {
//     ExitPoint,
//     Up,
//     Down,
// }

// #[derive(Clone, Copy)]
// pub enum ListOp<'a> {
//     Nav(ListDir),
//     New(&'a str, &'a str),
// }
// fn check() {
//     use crate::ls::*;
// }
// #[derive(Clone, Copy)]
// pub enum Signal<'a> {
//     Quit,
//     ChCtxt(Ctxt),
//     List(list::ListOp<'a>),
// }

// #[derive(Clone, Copy, PartialEq, Eq)]
// pub enum Ctxt {
//     Default,
//     List,
// }
// impl Ctxt {
//     pub fn process_input(&self, key: &KeyEvent) -> Option {
//         use utils::list::*;
//         use ctxt::*;
//         use list::*;
//         use signal::*;
//         match self {
//             Ctxt::Default => match key.code {
//                 KeyCode::Char('Q') | KeyCode::Char('q') => Some(Signal::Quit),
//                 KeyCode::Char('l') => Some(Signal::ChCtxt(Ctxt::List)),
//                 KeyCode::Left => Some(Signal::List(ListOp::Nav(ListDir::ExitPoint))),
//                 KeyCode::Up => Some(Signal::List(ListOp::Nav(ListDir::Up))),
//                 KeyCode::Down => Some(Signal::List(ListOp::Nav(ListDir::Down))),
//                 _ => None,
//             },
//             Ctxt::List => match key.code {
//                 KeyCode::Char('Q') => Some(Signal::Quit),
//                 KeyCode::Char('q') | KeyCode::Right | KeyCode::Esc => {
//                     Some(Signal::ChCtxt(Ctxt::Default))
//                 }
//                 KeyCode::Up => Some(Signal::List(ListOp::Nav(ListDir::Up))),
//                 KeyCode::Down => Some(Signal::List(ListOp::Nav(ListDir::Down))),
//                 KeyCode::Char('N') | KeyCode::Char('n') => {
//                     let new_title = "title";
//                     let new_desc = "desc";
//                     Some(Signal::List(ListOp::New(new_title, new_desc)))
//                 }
//                 _ => None,
//             },
//         }
//     }
// }

pub struct App<'a, B> {
    // terminal: Terminal<B>,
    items: StatefulList<(&'a str, usize)>,
    events: Vec<(&'a str, &'a str)>,
    // state: Option<ActiveBlock>,
    context: utils::context::Context,
    // queue: Vec<Signal>,
    phantom_data: std::marker::PhantomData<B>,
}
impl<'a, B: Backend> Default for App<'a, B> {
    fn default() -> Self {
        Self {
            // terminal,
            items: StatefulList::with_items(vec![
                ("Item0", 1),
                ("Item1", 2),
                ("Item2", 1),
                ("Item3", 3),
                ("Item4", 1),
                ("Item5", 4),
                ("Item6", 1),
                ("Item7", 3),
                ("Item8", 1),
                ("Item9", 6),
            ]),
            events: vec![
                ("Event1", "INFO"),
                ("Event2", "INFO"),
                ("Event3", "CRITICAL"),
                ("Event4", "ERROR"),
            ],
            // state: None,
            context: utils::Context::Default,
            // queue: vec![],
            phantom_data: std::marker::PhantomData::default(),
        }
    }
}
impl<'a, B: Backend> App<'a, B>
where
    B: io::Write,
{
    pub fn new() -> Self {
        Self {
            // terminal,
            items: StatefulList::with_items(vec![
                ("Item0", 1),
                ("Item1", 2),
                ("Item2", 1),
                ("Item3", 3),
                ("Item4", 1),
                ("Item5", 4),
                ("Item6", 1),
                ("Item7", 3),
                ("Item8", 1),
                ("Item9", 6),
            ]),
            events: vec![
                ("Event1", "INFO"),
                ("Event2", "INFO"),
                ("Event3", "CRITICAL"),
                ("Event4", "ERROR"),
            ],
            // state: None,
            context: utils::Context::Default,
            // queue: vec![],
            phantom_data: std::marker::PhantomData::default(),
        }
    }
    fn quit(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture,
        )?;
        terminal.show_cursor()?;
        Ok(())
    }

    fn process(
        &mut self,
        signal: &utils::Signal<'a>,
        terminal: &mut Terminal<B>,
    ) -> io::Result<()> {
        use utils::list::*;
        use utils::Context;
        use utils::Signal;
        match signal {
            Signal::Quit => self.quit(terminal),
            Signal::ChangeContext(ctxt) => match (&self.context, ctxt) {
                (Context::List, target_ctxt) if target_ctxt != &Context::List => {
                    self.context = *target_ctxt;
                    self.items.deselect()
                }
                (current_ctxt, target_ctxt) if current_ctxt == target_ctxt => {
                    panic!("Attempted circular context movement!")
                }
                (_, Context::List) => {
                    self.context = Context::List;
                    self.items.reselect()
                }
                (_, _) => panic!("Non-existent context movement!"),
            },
            Signal::ListSignal(Operation::Go(location)) => match &self.context {
                Context::List => match location {
                    Location::Up => self.items.prev(),
                    Location::Down => self.items.next(),
                    Location::ExitPoint => self.items.reselect(),
                },
                Context::Default => match location {
                    Location::Up => self.items.reselect_prev(),
                    Location::Down => self.items.reselect_next(),
                    Location::ExitPoint => self.items.reselect(),
                },
            },
            // Signal::ListNav(direction) => match &self.ctxt {
            //     Ctxt::List => match direction {
            //         ListDir::Up => self.items.prev(),
            //         ListDir::Down => self.items.next(),
            //         ListDir::ExitPoint => self.items.reselect(),
            //     },
            //     Ctxt::Default => {
            //         self.ctxt = Ctxt::List;
            //         match direction {
            //             ListDir::Up => self.items.reselect_prev(),
            //             ListDir::Down => self.items.reselect_next(),
            //             ListDir::ExitPoint => self.items.reselect(),
            //         }
            //     }
            //     _ => panic!("Signal::ListNav while not in list!"),
            // },
            Signal::ListSignal(Operation::New(title, _)) => match &self.context {
                Context::List => Ok(self.items.list.push((title, 1usize))),
                Context::Default => todo!(),
            },
        }
    }

    fn poll(&'a mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if let Some(signal) = self.context.process_input(&key) {
                self.process(&signal, terminal)
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    fn on_tick(&mut self) -> Result<(), io::Error> {
        let event = self.events.remove(0);
        self.events.push(event);
        Ok(())
    }

    fn ui(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(f.size());

        let items = self
            .items
            .list
            .iter()
            .map(|item| {
                let mut lines = vec![Spans::from(item.0)];
                for _ in 0..item.1 {
                    lines.push(Spans::from(Span::styled(
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                        Style::default().add_modifier(Modifier::ITALIC),
                    )));
                }
                ListItem::new(lines).style(Style::default().fg(Color::White))
            })
            .collect::<Vec<ListItem>>();
        let item_base_style = Style::default().fg(Color::White);
        let border_style = match &self.items.state.selected() {
            Some(_) => Style::default()
                .fg(Color::Red)
                .bg(Color::Black)
                .add_modifier(Modifier::RAPID_BLINK),
            None => Style::default()
                .fg(Color::LightRed)
                .bg(Color::DarkGray)
                .remove_modifier(Modifier::RAPID_BLINK),
        };
        let list = List::new(items)
            .block(
                Block::default()
                    .title("List")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White).bg(Color::Cyan)),
            )
            .style(Style::default().fg(Color::Black).bg(Color::Reset))
            .highlight_style(Style::default().fg(Color::White).bg(Color::LightCyan))
            .highlight_symbol(">>");
        f.render_stateful_widget(list, chunks[0], &mut self.items.state);

        let block = Block::default().title("Block 2").borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App<B>)
where
    B: io::Write,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.size());

    let items = app
        .items
        .list
        .iter()
        .map(|item| {
            let mut lines = vec![Spans::from(item.0)];
            for _ in 0..item.1 {
                lines.push(Spans::from(Span::styled(
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                    Style::default().add_modifier(Modifier::ITALIC),
                )));
            }
            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect::<Vec<ListItem>>();
    let item_base_style = Style::default().fg(Color::White);
    let border_style = match &app.items.state.selected() {
        Some(_) => Style::default()
            .fg(Color::Red)
            .bg(Color::Black)
            .add_modifier(Modifier::RAPID_BLINK),
        None => Style::default()
            .fg(Color::LightRed)
            .bg(Color::DarkGray)
            .remove_modifier(Modifier::RAPID_BLINK),
    };
    let list = List::new(items)
        .block(
            Block::default()
                .title("List")
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(item_base_style.add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");
    f.render_stateful_widget(list, chunks[0], &mut app.items.state);

    let block = Block::default().title("Block 2").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}

// #[derive(Clone, Copy)]
// pub enum ActiveBlock {
//     ListBlock,
//     Other,
// }

// impl ActiveBlock {
//     pub fn context_events(&self, app: &mut App, key: KeyEvent) -> io::Result<()> {
//         match self {
//             ActiveBlock::ListBlock => match key.code {
//                 // IMPLEMENT A SIGNAL QUEUE; FIRE QUIT SIGNAL; PROCESS ON TICK
//                 KeyCode::Char('q') => (),
//                 KeyCode::Right => app.items.deselect(),
//                 KeyCode::Down => app.items.next(),
//                 KeyCode::Up => app.items.prev(),
//                 _ => {}
//             },
//             ActiveBlock::Other => match key.code {
//                 KeyCode::Char('q') => return Ok(()),
//                 KeyCode::Left => app.items.reselect(),
//                 KeyCode::Down => app.items.reselect_next(),
//                 KeyCode::Up => app.items.reselect_prev(),
//                 _ => {}
//             },
//             _ => match key.code {},
//         }
//     }
// }

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &'static mut App<B>,
    tick_rate: Duration,
) -> io::Result<()>
where
    B: io::Write,
{
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| app.ui(f))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            app.poll(terminal)?;
            // app.roll(terminal)?;
            // if let Event::Key(key) = event::read()? {
            //     if let Some(active_block) = app.state {
            //         match active_block {
            //             ActiveBlock::ListBlock => match key.code {
            //                 KeyCode::Char('q') => return Ok(()),
            //                 KeyCode::Right => app.items.deselect(),
            //                 KeyCode::Down => app.items.next(),
            //                 KeyCode::Up => app.items.prev(),
            //                 _ => {}
            //             },
            //             ActiveBlock::Other => match key.code {
            //                 KeyCode::Char('q') => return Ok(()),
            //                 KeyCode::Left => app.items.reselect(),
            //                 KeyCode::Down => app.items.reselect_next(),
            //                 KeyCode::Up => app.items.reselect_prev(),
            //                 _ => {}
            //             },
            //             _ => match key.code {
            //                 KeyCode::Char('q') => return Ok(()),
            //                 _ => {}
            //             },
            //         }
            //     }
            // }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick()?;
            last_tick = Instant::now();
        }
    }
}
