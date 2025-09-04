use ratatui::crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::{error::Error, fs, io};

mod app;
mod app_methods;
mod config;
mod constants;
mod file_ops;
mod modes;
mod ui;

use app::App;
use constants::WELCOME_FILE_CONTENT;
use modes::Mode;
use ui::ui;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    if fs::metadata("welcome.md").is_err() {
        fs::write("welcome.md", WELCOME_FILE_CONTENT)?;
    }

    let mut terminal = setup_terminal()?;
    let mut app = App::new();
    run(&mut terminal, &mut app)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run<'a>(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App<'a>,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            let current_mode = app.mode;
            match current_mode {
                Mode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('n') => app.mode = Mode::Naming,
                    KeyCode::Char('c') => app.enter_directory_browser(false),
                    KeyCode::Char('T') => app.start_template_workflow(),
                    KeyCode::Char('/') => app.enter_search_mode(),
                    KeyCode::Char('s') => app.enter_settings(),
                    KeyCode::Char('d') => app.start_delete_confirmation(),
                    KeyCode::Char('m') => app.start_move_selection(),
                    KeyCode::Down => app.select_next(),
                    KeyCode::Up => app.select_previous(),
                    KeyCode::Left => app.navigate_up_directory(),
                    KeyCode::Right | KeyCode::Enter => app.start_editing(),
                    _ => {}
                },
                Mode::Editing => match key.code {
                    KeyCode::Esc => app.stop_editing(),
                    _ => {
                        if let Some(editor) = app.editor.as_mut() {
                            editor.input(key);
                        }
                    }
                },
                Mode::Naming => match key.code {
                    KeyCode::Enter => app.create_new_note(),
                    KeyCode::Esc => {
                        app.filename_input.clear();
                        app.pending_template = None;
                        app.mode = Mode::Normal;
                    }
                    KeyCode::Char(c) => {
                        app.filename_input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.filename_input.pop();
                    }
                    _ => {}
                },
                Mode::ChangingDirectory => match key.code {
                    KeyCode::Esc => app.mode = Mode::Normal,
                    KeyCode::Char('s') => app.set_new_root(),
                    KeyCode::Enter => app.select_browser_entry(),
                    KeyCode::Down => app.select_next(),
                    KeyCode::Up => app.select_previous(),
                    _ => {}
                },
                Mode::SelectingTemplateFolder => match key.code {
                    KeyCode::Esc => app.mode = Mode::Normal,
                    KeyCode::Char('s') => app.set_template_root(),
                    KeyCode::Enter => app.select_browser_entry(),
                    KeyCode::Down => app.select_next(),
                    KeyCode::Up => app.select_previous(),
                    _ => {}
                },
                Mode::SelectingTemplate => match key.code {
                    KeyCode::Esc => app.mode = Mode::Normal,
                    KeyCode::Enter => app.select_template(),
                    KeyCode::Down => app.select_next(),
                    KeyCode::Up => app.select_previous(),
                    _ => {}
                },
                Mode::Search => match key.code {
                    KeyCode::Esc => app.exit_search_mode(),
                    KeyCode::Right | KeyCode::Enter => app.start_editing(),
                    KeyCode::Down => app.select_next(),
                    KeyCode::Up => app.select_previous(),
                    KeyCode::Left => app.navigate_up_directory(),
                    KeyCode::Char('d') => app.start_delete_confirmation(),
                    KeyCode::Char('m') => app.start_move_selection(),
                    KeyCode::Char(c) => {
                        app.search_input.push(c);
                        app.update_filtered_files();
                    }
                    KeyCode::Backspace => {
                        app.search_input.pop();
                        app.update_filtered_files();
                    }
                    _ => {}
                },
                Mode::ConfirmingDelete => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => app.confirm_delete(),
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => app.cancel_operation(),
                    _ => {}
                },
                Mode::SelectingMoveDestination => match key.code {
                    KeyCode::Esc => app.cancel_operation(),
                    KeyCode::Enter => app.execute_move(),
                    KeyCode::Down => app.move_selection_next(),
                    KeyCode::Up => app.move_selection_previous(),
                    _ => {}
                },
                Mode::Settings => match key.code {
                    KeyCode::Esc => app.exit_settings(),
                    KeyCode::Enter => {
                        app.apply_color_scheme();
                        app.exit_settings();
                    }
                    KeyCode::Down => app.settings_next(),
                    KeyCode::Up => app.settings_previous(),
                    _ => {}
                },
            }
        }
    }
}