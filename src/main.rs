pub mod ctxt;
pub mod listop;
pub mod ls;
pub mod signal;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ocaml_notebook::*;
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};
fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(50);
    let mut app = App::new();
    run_app(&mut terminal, &mut app, tick_rate)?;
    // terminal.draw(|f| {
    //     let size = f.size();
    //     let block = Block::default().title("Block").borders(Borders::ALL);
    //     ui(f);
    //     // f.render_widget(block, size);
    // })?;

    // thread::sleep(Duration::from_millis(5000));
    // for _ in 0..10 {
    //     terminal.show_cursor()?;
    //     thread::sleep(Duration::from_millis(500));
    //     terminal.hide_cursor()?;
    //     thread::sleep(Duration::from_millis(500))
    // }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
