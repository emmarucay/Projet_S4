mod models;
mod logic;
mod filters;
mod ui;
mod progress;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use models::Manager;
use logic::sort_tasks;
use ui::{AppState, Screen};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut manager = Manager::new();
    let mut state   = AppState::new();
    let mut filter_cat = String::new();

    loop {
        terminal.draw(|f| {
            match &state.screen {
                Screen::Welcome          => ui::draw_welcome(f, &state),
                Screen::TaskList         => ui::draw_task_list(f, &manager.tasks, &mut state),
                Screen::FilterByCategory => ui::draw_filter_screen(f, &manager.tasks, &filter_cat),
                Screen::Stats            => ui::draw_stats(f, &manager.tasks),
                Screen::AddTask          => {}
            }
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match &state.screen {

                Screen::Welcome => match key.code {
                    KeyCode::Up   => { if state.selected_menu > 0 { state.selected_menu -= 1; } }
                    KeyCode::Down => { if state.selected_menu < 4 { state.selected_menu += 1; } }
                    KeyCode::Enter => match state.selected_menu {
                        0 => state.screen = Screen::TaskList,
                        1 => state.screen = Screen::AddTask,
                        2 => {
                            disable_raw_mode()?;
                            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                            print!("\n  \x1b[35mCategory to filter\x1b[0m : ");
                            io::Write::flush(&mut io::stdout())?;
                            let mut cat = String::new();
                            io::BufRead::read_line(&mut io::BufReader::new(io::stdin()), &mut cat)?;
                            filter_cat = cat.trim().to_string();
                            enable_raw_mode()?;
                            execute!(terminal.backend_mut(), EnterAlternateScreen)?;
                            terminal.clear()?;

                            if filter_cat.is_empty() {
                                state.screen = Screen::TaskList;
                            } else {
                                state.screen = Screen::FilterByCategory;
                            }
                        }
                        3 => state.screen = Screen::Stats,
                        4 => break,
                        _ => {}
                    },
                    KeyCode::Char('q') => break,
                    _ => {}
                },

                Screen::TaskList => match key.code {
                    KeyCode::Esc | KeyCode::Char('b') => state.screen = Screen::Welcome,
                    KeyCode::Char('q') => break,

                    KeyCode::Up => {
                        let sel = state.list_state.selected().unwrap_or(0);
                        if sel > 0 { state.list_state.select(Some(sel - 1)); }
                    }
                    KeyCode::Down => {
                        let sel = state.list_state.selected().unwrap_or(0);
                        if sel + 1 < manager.tasks.len() { state.list_state.select(Some(sel + 1)); }
                    }

                    KeyCode::Char('a') => state.screen = Screen::AddTask,

                    KeyCode::Char('s') => {
                        sort_tasks(&mut manager.tasks);
                        if manager.tasks.is_empty() {
                            state.list_state.select(None);
                        } else {
                            state.list_state.select(Some(0));
                        }
                    }

                  KeyCode::Char('f') => {
                        disable_raw_mode()?;
                        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                        print!("\n  \x1b[35mCategory to filter\x1b[0m : ");
                        io::Write::flush(&mut io::stdout())?;
                        let mut cat = String::new();
                        io::stdin().read_line(&mut cat)?;
                        filter_cat = cat.trim().to_string();
                        enable_raw_mode()?;
                        execute!(terminal.backend_mut(), EnterAlternateScreen)?;
                        terminal.clear()?;

                        if filter_cat.is_empty() { //secure test
                            state.screen = Screen::TaskList;
                        } else {
                            state.screen = Screen::FilterByCategory;
                        }
                    }

                    KeyCode::Char('e') => {
                        if let Some(idx) = state.list_state.selected() {
                            if idx < manager.tasks.len() {
                                disable_raw_mode()?;
                                execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                                ui::prompt_modify_task_tui(&mut manager.tasks[idx]);
                                enable_raw_mode()?;
                                execute!(terminal.backend_mut(), EnterAlternateScreen)?;
                                terminal.clear()?;
                            }
                        }
                    }

                    // Espace = marquer terminée/à faire (feature Farah)
                    KeyCode::Char(' ') => {
                        if let Some(idx) = state.list_state.selected() {
                            if idx < manager.tasks.len() {
                                manager.tasks[idx].completed = !manager.tasks[idx].completed;
                            }
                        }
                    }

                    KeyCode::Delete | KeyCode::Backspace => {
                        if let Some(idx) = state.list_state.selected() {
                            if idx < manager.tasks.len() {
                                manager.tasks.remove(idx);
                                let new_sel = if idx > 0 { idx - 1 } else { 0 };
                                state.list_state.select(if manager.tasks.is_empty() { None } else { Some(new_sel) });
                            }
                        }
                    }
                    _ => {}
                },

                Screen::AddTask => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    if let Some(task) = ui::prompt_new_task_tui() {
                        manager.add_task(task);
                    }
                    enable_raw_mode()?;
                    execute!(terminal.backend_mut(), EnterAlternateScreen)?;
                    terminal.clear()?;
                    state.screen = Screen::TaskList;
                }

                Screen::FilterByCategory | Screen::Stats => match key.code {
                    KeyCode::Esc | KeyCode::Char('b') | KeyCode::Char('q') => {
                        state.screen = Screen::TaskList;
                    }
                    _ => {}
                },
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    println!("  See you soon ! 🌸");
    Ok(())
}
