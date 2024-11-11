mod app;
mod ui;

use ratatui::crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    enable_raw_mode,
    disable_raw_mode,
    EnterAlternateScreen,
    LeaveAlternateScreen
};
use ratatui::prelude::{Backend, CrosstermBackend};
use ratatui::Terminal;
use std::error::Error;
use std::io;
use app::{App, CurrentScreen, CurrentlyEditing};
// use ui;

fn main() -> Result<(), Box<dyn Error>> {
    
    // setup terminal
    enable_raw_mode()?; // raw mode disables terminal's default input buffering
    let mut stderr = io::stderr();
    // alternate screen is a secondary blank screen which our program runs on
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create and run the app
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal when closing the app
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Event::Key(key) = event::read()? {
            // ignores key release events
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Key);
                    },
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    },
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    },
                    KeyCode::Char('q') | KeyCode::Char('n') => {
                        return Ok(false);
                    },
                    _ => {}
                },
                
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Enter => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.currently_editing = Some(CurrentlyEditing::Value);
                                    },
                                    CurrentlyEditing::Value => {
                                        app.save_key_value();
                                        app.current_screen = CurrentScreen::Main;
                                    }
                                }
                            }
                        },

                        KeyCode::Backspace => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.key_input.pop();
                                    },
                                    CurrentlyEditing::Value => {
                                        app.value_input.pop();
                                    }
                                }
                            }
                        },

                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Exiting;
                            app.currently_editing = None;
                        },

                        KeyCode::Tab => {
                            app.toggle_editing();
                        },

                        KeyCode::Char(value) => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.key_input.push(value);
                                    },
                                    CurrentlyEditing::Value => {
                                        app.value_input.push(value);
                                    }
                                }
                            }
                        },

                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}
