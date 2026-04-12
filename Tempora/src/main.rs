mod models;
mod logic;
mod filters;
mod ui;
mod storage;
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

    let mut manager = storage::load_from_file("tasks.json").unwrap_or_else(|_|
        {
            Manager::new()
        });
    let mut state = AppState::new();
    let mut filter_cat = String::new();

    loop {
        terminal.draw(|f| {
            match &state.screen {
                Screen::Welcome => ui::draw_welcome(f, &state),
                Screen::TaskList => ui::draw_task_list(f, &manager.tasks, &mut state),
                Screen::FilterByCategory => ui::draw_filter_screen(f, &manager.tasks, &filter_cat),
                Screen::Stats => ui::draw_stats(f, &manager.tasks),
                Screen::Calendar => ui::draw_calendar(f, &manager.tasks, &manager.events, &state.cal_state),
                Screen::DayDetail => {
                    if let Some(day) = state.cal_state.selected_day {
                        let date = chrono::NaiveDate::from_ymd_opt(
                            state.cal_state.year,
                            state.cal_state.month,
                            day,
                        ).unwrap();
                        ui::draw_day_detail(f, &manager.tasks, &manager.events, date);
                    }
                }
                Screen::AddTask => {}
                Screen::AddEvent => {}
            }
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match &state.screen {

                // ───── MENU  ─────
                Screen::Welcome => match key.code {
                    KeyCode::Up => {
                        if state.selected_menu > 0 {
                            state.selected_menu -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if state.selected_menu < 6 {
                            state.selected_menu += 1;
                        }
                    }

                    KeyCode::Enter => match state.selected_menu {

                        0 => state.screen = Screen::TaskList,

                        // ADD TASK
                        1 => {
                            disable_raw_mode()?;
                            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

                            if let Some(task) = ui::prompt_new_task_tui() {
                                manager.add_task(task);
                                println!("\x1b[32m\nEvent added successfully\x1b[0m");
                            }

                            std::thread::sleep(std::time::Duration::from_millis(1200));

                            enable_raw_mode()?;
                            execute!(terminal.backend_mut(), EnterAlternateScreen)?;
                            terminal.clear()?;
                        }

                        // ADD EVENT (TA PARTIE)
                        2 => {
                            disable_raw_mode()?;
                            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

                            loop {
                                if let Some(event) = ui::prompt_new_event_tui() {

                                    match manager.add_event(event) {

                                        Ok(_) => {
                                            println!("\x1b[32m\nEvent added successfully\x1b[0m");
                                            std::thread::sleep(std::time::Duration::from_millis(1200));
                                            break;
                                        }

                                        Err(_e) => {
                                            println!("\x1b[31m");
                                            println!("Unable to add this event.");
                                            println!("\x1b[0m");

                                            std::thread::sleep(std::time::Duration::from_millis(2500));
                                        }
                                    }

                                } else {
                                    println!("Event canceled");
                                    break;
                                }
                            }

                            enable_raw_mode()?;
                            execute!(terminal.backend_mut(), EnterAlternateScreen)?;
                            terminal.clear()?;
                        }

                        // FILTER
                        3 => {
                            disable_raw_mode()?;
                            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

                            print!("\nCategory to filter: ");
                            io::Write::flush(&mut io::stdout())?;

                            let mut cat = String::new();
                            io::stdin().read_line(&mut cat)?;
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

                        4 => state.screen = Screen::Calendar,
                        5 => state.screen = Screen::Stats,
                        6 => break,
                        _ => {}
                    },

                    KeyCode::Char('q') => break,
                    _ => {}
                },

                // ───── TASK LIST ─────
                Screen::TaskList => match key.code {
                    KeyCode::Esc | KeyCode::Char('b') => state.screen = Screen::Welcome,
                    KeyCode::Char('q') => break,

                    KeyCode::Up => {
                        let sel = state.list_state.selected().unwrap_or(0);
                        if sel > 0 {
                            state.list_state.select(Some(sel - 1));
                        }
                    }

                    KeyCode::Down => {
                        let sel = state.list_state.selected().unwrap_or(0);
                        if sel + 1 < manager.tasks.len() {
                            state.list_state.select(Some(sel + 1));
                        }
                    }

                    KeyCode::Char('a') => {
                        disable_raw_mode()?;
                        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

                        if let Some(task) = ui::prompt_new_task_tui() {
                            manager.add_task(task);
                            println!("\x1b[32m\nEvent added successfully.\x1b[0m");
                        }

                        std::thread::sleep(std::time::Duration::from_millis(1200));

                        enable_raw_mode()?;
                        execute!(terminal.backend_mut(), EnterAlternateScreen)?;
                        terminal.clear()?;
                    }

                    KeyCode::Char('s') => {
                        sort_tasks(&mut manager.tasks);
                        state.list_state.select(Some(0));
                    }

                    KeyCode::Delete | KeyCode::Backspace => {
                        if let Some(idx) = state.list_state.selected() {
                            if idx < manager.tasks.len() {
                                manager.tasks.remove(idx);
                            }
                        }
                    }

                    _ => {}
                },

                // ───── CALENDAR ─────
                Screen::Calendar => match key.code {
                    KeyCode::Esc | KeyCode::Char('b') => state.screen = Screen::Welcome,
                    KeyCode::Char('q') => break,

                    KeyCode::Char('h') => state.cal_state.prev_month(),
                    KeyCode::Char('l') => state.cal_state.next_month(),
                    KeyCode::Up => state.cal_state.move_day(-7),
                    KeyCode::Down => state.cal_state.move_day(7),
                    KeyCode::Left => state.cal_state.move_day(-1),
                    KeyCode::Right => state.cal_state.move_day(1),

                    KeyCode::Char('e') => {
                        disable_raw_mode()?;
                        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

                        loop {
                            if let Some(event) = ui::prompt_new_event_tui() {

                                match manager.add_event(event) {

                                    Ok(_) => {
                                        println!("\x1b[32m\nEvent added successfully\x1b[0m");
                                        std::thread::sleep(std::time::Duration::from_millis(1200));
                                        break;
                                    }

                                    Err(_e) => {
                                        println!("\x1b[31m");
                                        println!("Unable to add this event.");
                                        println!("\x1b[0m");

                                        std::thread::sleep(std::time::Duration::from_millis(2500));
                                    }
                                }

                            } else {
                                println!("Event canceled");
                                break;
                            }
                        }

                        enable_raw_mode()?;
                        execute!(terminal.backend_mut(), EnterAlternateScreen)?;
                        terminal.clear()?;
                    }

                    KeyCode::Enter => state.screen = Screen::DayDetail,
                    _ => {}
                },

                // ───── DAY DETAIL ─────
                Screen::DayDetail => match key.code {
                    KeyCode::Esc | KeyCode::Char('b') => state.screen = Screen::Calendar,
                    KeyCode::Char('q') => break,
                    _ => {}
                },

                // ───── AUTRES ─────
                Screen::FilterByCategory | Screen::Stats => match key.code {
                    KeyCode::Esc | KeyCode::Char('b') => state.screen = Screen::TaskList,
                    KeyCode::Char('q') => break,
                    _ => {}
                },

                Screen::AddTask => state.screen = Screen::TaskList,
                Screen::AddEvent => state.screen = Screen::TaskList,
            }
        }
    }
    
    if let Err(e) = storage::save_to_file(&manager, "tasks.json")
    {
        eprintln!("Erreur lors de la sauvegarde : {}", e);
    } 

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    println!("See you soon! 🌸");
    Ok(())
}
