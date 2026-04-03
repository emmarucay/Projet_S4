use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};
use chrono::{Duration, NaiveDateTime};
use crate::models::{Task, Priority};
use crate::filters::filter_by_category;

// ─── PALETTE ─────────────────────────────────────────────────────────────────
// Simulating a pink vibe with available terminal colors
const PINK:       Color = Color::Magenta;
const DEEP_PINK:  Color = Color::Red;       // for urgent tasks
const SOFT:       Color = Color::LightMagenta;
const MUTED:      Color = Color::DarkGray;
const WHITE:      Color = Color::White;
const LAVENDER:   Color = Color::LightBlue;
#[allow(dead_code)]
const BG:         Color = Color::Reset;

// ─── APP STATE ───────────────────────────────────────────────────────────────
pub enum Screen {
    Welcome,
    TaskList,
    AddTask,
    AddEvent,
    FilterByCategory,
    Stats,
    Calendar,
    DayDetail,
}

pub struct AppState {
    pub screen: Screen,
    pub list_state: ListState,
    pub selected_menu: usize,   // welcome menu index
    pub cal_state: CalendarState,
}

impl AppState {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            screen: Screen::Welcome,
            list_state,
            selected_menu: 0,
            cal_state: CalendarState::new(),
        }
    }
}

// ─── STYLE HELPERS ───────────────────────────────────────────────────────────
fn priority_color(p: &Priority) -> Color {
    match p {
        Priority::Five  => DEEP_PINK,
        Priority::Four  => Color::LightRed,
        Priority::Three => PINK,
        Priority::Two   => SOFT,
        Priority::One   => MUTED,
    }
}

fn priority_label(p: &Priority) -> &'static str {
    match p {
        Priority::Five  => "★★★★★ Urgent",
        Priority::Four  => "★★★★☆ High",
        Priority::Three => "★★★☆☆ Medium",
        Priority::Two   => "★★☆☆☆ Low",
        Priority::One   => "★☆☆☆☆ Minimal",
    }
}

fn priority_dot(p: &Priority) -> &'static str {
    match p {
        Priority::Five | Priority::Four | Priority::Three => "●",
        Priority::Two  | Priority::One                    => "○",
    }
}
#[allow(dead_code)]
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

// ─── WELCOME SCREEN ───────────────────────────────────────────────────────────
#[allow(dead_code)]
pub fn draw_welcome(f: &mut Frame, state: &AppState) {
    let area = f.size();

    // Layout: title at top, options in middle, footer at bottom
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(50),
            Constraint::Percentage(20),
        ])
        .split(area);

    // ── TITLE ──
    let title_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  ✦  ", Style::default().fg(SOFT)),
            Span::styled("T E M P O R A", Style::default().fg(PINK).add_modifier(Modifier::BOLD)),
            Span::styled("  ✦  ", Style::default().fg(SOFT)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  Your personal organizer  ",
                Style::default().fg(MUTED),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled("  · · · · · · · · · · · · · · · · · · · ·  ", Style::default().fg(SOFT))),
    ];

    let title = Paragraph::new(title_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(title, chunks[0]);

    let menu_items = vec![
        ("  ✅  My tasks        ", "View, sort and manage your tasks"),
        ("  ✍   Add task        ", "Create a new task"),
        ("  🗓   Add event       ", "Create a new event"),
        ("  🔍  Filter           ", "Find by category"),
        ("  📅  Calendar         ", "Monthly view of tasks & events"),
        ("  📊  Stats            ", "Your progress"),
        ("  ✕   Quit             ", ""),
    ];

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ])
        .split(chunks[1]);

    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, (label, desc))| {
            let is_selected = i == state.selected_menu;
            let prefix = if is_selected { "▶ " } else { "  " };
            let style = if is_selected {
                Style::default().fg(WHITE).bg(PINK).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(SOFT)
            };
            ListItem::new(vec![
                Line::from(Span::styled(format!("{}{}", prefix, label), style)),
                Line::from(Span::styled(format!("   {}", desc), Style::default().fg(MUTED))),
                Line::from(""),
            ])
        })
        .collect();

    let menu = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(SOFT))
                .title(Span::styled(" Menu ", Style::default().fg(PINK).add_modifier(Modifier::BOLD))),
        );
    f.render_widget(menu, cols[1]);

    let footer = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  ↑↓ navigate  ", Style::default().fg(MUTED)),
            Span::styled("  Enter select  ", Style::default().fg(SOFT)),
            Span::styled("  q quit  ", Style::default().fg(MUTED)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  Made with ♡ by Marilyn · Emma · Lisa · Farah  ",
            Style::default().fg(SOFT).add_modifier(Modifier::ITALIC),
        )),
    ])
    .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
}

// ─── TASK LIST SCREEN ────────────────────────────────────────────────────────
#[allow(dead_code)]
pub fn draw_task_list(f: &mut Frame, tasks: &[Task], state: &mut AppState) {
    let area = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // header
            Constraint::Min(0),      // list
            Constraint::Length(3),   // footer
        ])
        .split(area);

    let header = Paragraph::new(vec![Line::from(vec![
        Span::styled("  ✦ TEMPORA  ", Style::default().fg(PINK).add_modifier(Modifier::BOLD)),
        Span::styled("›  ", Style::default().fg(MUTED)),
        Span::styled("My tasks", Style::default().fg(WHITE).add_modifier(Modifier::BOLD)),
        Span::styled(
            format!("  ({} task{})", tasks.len(), if tasks.len() > 1 { "s" } else { "" }),
            Style::default().fg(MUTED),
        ),
    ])])
    .block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(SOFT)));
    f.render_widget(header, chunks[0]);

    // List
    if tasks.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "  🌸  No tasks yet",
                Style::default().fg(MUTED).add_modifier(Modifier::ITALIC),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "  Press 'a' to add one!",
                Style::default().fg(SOFT),
            )]),
        ])
        .block(Block::default().borders(Borders::NONE));
        f.render_widget(empty, chunks[1]);
    } else {
        let items: Vec<ListItem> = tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                let dot_color = priority_color(&task.priority);
                let is_sel = state.list_state.selected() == Some(i);
                let prefix = if is_sel { "▶ " } else { "  " };

                // Nom en noir + barré si terminée
                let name_style = if task.completed {
                    Style::default().fg(MUTED).add_modifier(Modifier::CROSSED_OUT)
                } else {
                    Style::default().fg(WHITE).add_modifier(Modifier::BOLD)
                };

                let statut = if task.completed { " ✓" } else { "" };

                ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(prefix, Style::default().fg(PINK)),
                        Span::styled(priority_dot(&task.priority), Style::default().fg(dot_color)),
                        Span::raw("  "),
                        Span::styled(format!("{}{}", task.name, statut), name_style),
                        Span::raw("   "),
                        Span::styled(priority_label(&task.priority), Style::default().fg(dot_color)),
                    ]),
                    Line::from(vec![
                        Span::raw("     "),
                        Span::styled(
                            task.description.chars().take(60).collect::<String>(),
                            Style::default().fg(MUTED),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("     "),
                        Span::styled(
                            format!("📅 {}  ", task.deadline.format("%d/%m/%Y %H:%M")),
                            Style::default().fg(LAVENDER),
                        ),
                        Span::styled(task.categories.join(" · "), Style::default().fg(SOFT)),
                    ]),
                    Line::from(Span::styled(
                        "  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─",
                        Style::default().fg(Color::DarkGray),
                    )),
                ])
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::NONE)
                    .padding(ratatui::widgets::Padding::horizontal(1)),
            )
            .highlight_style(Style::default().bg(Color::Rgb(80, 20, 60)));

        f.render_stateful_widget(list, chunks[1], &mut state.list_state);
    }

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("  a add  ", Style::default().fg(MUTED)),
        Span::styled("  s sort  ", Style::default().fg(MUTED)),
        Span::styled("  f filter  ", Style::default().fg(MUTED)),
        Span::styled("  e edit  ", Style::default().fg(MUTED)),
        Span::styled("  del delete  ", Style::default().fg(MUTED)),
        Span::styled("  esc back  ", Style::default().fg(MUTED)),
    ]))
    .block(Block::default().borders(Borders::TOP).border_style(Style::default().fg(SOFT)));
    f.render_widget(footer, chunks[2]);
}

// ─── POPUP: ADD A TASK ───────────────────────────────────────────────────────
// This function displays a recap in a popup after classic text input.
// Interactive input stays in text mode (stdin) because ratatui and stdin
// don't mix easily without an async lib. We exit raw mode,
// read input, then come back.
#[allow(dead_code)]
pub fn prompt_new_task_tui() -> Option<Task> {
    // Temporarily exit raw mode to read from terminal
    disable_raw_mode().ok();
    execute!(io::stdout(), LeaveAlternateScreen).ok();

    println!();
    println!("  \x1b[35m✦  NEW TASK\x1b[0m");
    println!("  \x1b[90m─────────────────────────────\x1b[0m");

    let name = read_field("  Task name", None);
    if name.is_empty() {
        restore_tui();
        return None;
    }

    let desc  = read_field("  Description", None);
    let prio  = read_priority();
    let cats  = read_field("  Categories (comma-separated)", Some("General"));
    let date  = read_deadline();

    let categories: Vec<String> = cats
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    restore_tui();

    Some(Task {
        name,
        description: desc,
        duration: Duration::minutes(30),
        priority: prio,
        deadline: date,
        categories,
        completed: false,
    })
}

// ─── MODIFIER UNE TÂCHE ──────────────────────────────────────────────────────
#[allow(dead_code)]
pub fn prompt_modify_task_tui(task: &mut Task) {
    disable_raw_mode().ok();
    execute!(io::stdout(), LeaveAlternateScreen).ok();

    println!();
    println!("  \x1b[35m✦  EDIT: {}\x1b[0m", task.name);
    println!("  \x1b[90m─────────────────────────────\x1b[0m");
    println!("  \x1b[90m(Enter = keep current value)\x1b[0m");
    println!();

    let new_name = read_field(&format!("  Name [{}]", task.name), None);
    if !new_name.is_empty() {
        task.name = new_name;
    }

    let new_desc = read_field(&format!("  Description [{}]", task.description), None);
    if !new_desc.is_empty() {
        task.description = new_desc;
    }

    println!("  Current priority: \x1b[35m{}\x1b[0m", priority_label(&task.priority));
    let new_prio_str = read_field("  New priority (1-5, Enter = keep)", None);
    if !new_prio_str.is_empty() {
        task.priority = parse_priority(&new_prio_str);
    }

    let cats_str = task.categories.join(", ");
    let new_cats = read_field(&format!("  Categories [{}]", cats_str), None);
    if !new_cats.is_empty() {
        task.categories = new_cats
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
    }

    let current_status = if task.completed { "yes" } else { "no" };
    let new_completed = read_field(
        &format!("  Completed? (yes/no) [{}]", current_status),
        None,
    );

    if !new_completed.is_empty() {
        match new_completed.trim().to_lowercase().as_str() {
            "yes" | "y" | "true" | "1" => task.completed = true,
            "no" | "n" | "false" | "0" => task.completed = false,
            _ => {}
        }
    }

    println!("\n  \x1b[35m✦ Task updated!\x1b[0m");
    std::thread::sleep(std::time::Duration::from_millis(800));
    restore_tui();
}

// helper : lit une date au format DD/MM/YYYY HH:MM
fn read_datetime(prompt: &str, default: &str) -> NaiveDateTime {
    let s = read_field(prompt, Some(default));
    NaiveDateTime::parse_from_str(&s, "%d/%m/%Y %H:%M")
        .unwrap_or_else(|_| {
            println!("  \x1b[90mInvalid format → using default\x1b[0m");
            NaiveDateTime::parse_from_str(default, "%d/%m/%Y %H:%M").unwrap()
        })
}

// ─── AJOUTER UN ÉVÉNEMENT ────────────────────────────────────────────────────
pub fn prompt_new_event_tui() -> Option<crate::models::Event> {
    disable_raw_mode().ok();
    execute!(io::stdout(), LeaveAlternateScreen).ok();

    println!();
    println!("  \x1b[35m✦  NEW EVENT\x1b[0m");
    println!("  \x1b[90m─────────────────────────────\x1b[0m");

    let name = read_field("  Event name", None);
    if name.is_empty() {
        restore_tui();
        return None;
    }

    let desc = read_field("  Description", None);
    let cats = read_field("  Categories (comma-separated)", Some("General"));
    let categories: Vec<String> = cats
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    println!("  \x1b[90mStart date/time:\x1b[0m");
    let start = read_datetime("  Start (DD/MM/YYYY HH:MM)", "01/01/2026 09:00");

    println!("  \x1b[90mEnd date/time:\x1b[0m");
    let end = read_datetime("  End   (DD/MM/YYYY HH:MM)", "01/01/2026 10:00");

    if end <= start {
        println!("  \x1b[31m⚠ End must be after start. Event not saved.\x1b[0m");
        std::thread::sleep(std::time::Duration::from_millis(1200));
        restore_tui();
        return None;
    }

    let prio = read_priority();
    restore_tui();

    Some(crate::models::Event {
        name,
        description: desc,
        start,
        end,
        priority: prio,
        categories,
    })
}

// ─── FILTER SCREEN ───────────────────────────────────────────────────────────
#[allow(dead_code)]
pub fn draw_filter_screen(f: &mut Frame, tasks: &[Task], category: &str) {
    let area = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    let header = Paragraph::new(Line::from(vec![
        Span::styled("  ✦ TEMPORA  ", Style::default().fg(PINK).add_modifier(Modifier::BOLD)),
        Span::styled("›  ", Style::default().fg(MUTED)),
        Span::styled("Filter: ", Style::default().fg(WHITE)),
        Span::styled(category, Style::default().fg(PINK).add_modifier(Modifier::BOLD)),
    ]))
    .block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(SOFT)));
    f.render_widget(header, chunks[0]);

    let filtered = filter_by_category(tasks, category);

    if filtered.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                format!("  No tasks in category «{}»", category),
                Style::default().fg(MUTED).add_modifier(Modifier::ITALIC),
            )),
        ]);
        f.render_widget(empty, chunks[1]);
    } else {
        let items: Vec<ListItem> = filtered.iter().map(|t| {
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled("  ● ", Style::default().fg(priority_color(&t.priority))),
                    Span::styled(t.name.clone(), Style::default().fg(Color::Black).add_modifier(Modifier::BOLD)),
                    Span::raw("   "),
                    Span::styled(priority_label(&t.priority), Style::default().fg(priority_color(&t.priority))),
                ]),
                Line::from(vec![
                    Span::raw("     "),
                    Span::styled(format!("📅 {}", t.deadline.format("%d/%m/%Y %H:%M")), Style::default().fg(LAVENDER)),
                ]),
                Line::from(""),
            ])
        }).collect();

        let list = List::new(items).block(Block::default().borders(Borders::NONE));
        f.render_widget(list, chunks[1]);
    }

    let footer = Paragraph::new(Line::from(Span::styled(
        "  esc back",
        Style::default().fg(MUTED),
    )))
    .block(Block::default().borders(Borders::TOP).border_style(Style::default().fg(SOFT)));
    f.render_widget(footer, chunks[2]);
}

// ─── STATS SCREEN ────────────────────────────────────────────────────────────
#[allow(dead_code)]
pub fn draw_stats(f: &mut Frame, tasks: &[Task]) {
    let area = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    let header = Paragraph::new(Line::from(vec![
        Span::styled("  ✦ TEMPORA  ", Style::default().fg(PINK).add_modifier(Modifier::BOLD)),
        Span::styled("›  ", Style::default().fg(MUTED)),
        Span::styled("Stats", Style::default().fg(WHITE).add_modifier(Modifier::BOLD)),
    ]))
    .block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(SOFT)));
    f.render_widget(header, chunks[0]);

    let total     = tasks.len();
    let terminees = tasks.iter().filter(|t| t.completed).count();
    let urgentes  = tasks.iter().filter(|t| matches!(t.priority, Priority::Five)).count();
    let categories: std::collections::HashSet<&str> = tasks
        .iter()
        .flat_map(|t| t.categories.iter().map(|c| c.as_str()))
        .collect();

    // Progression (feature Farah)
   let _progression = if total > 0 { (terminees as f64 / total as f64) * 100.0 } else { 0.0 };

    let prio_counts: Vec<(String, usize, Color)> = vec![
        ("★★★★★ Urgent ".to_string(), tasks.iter().filter(|t| matches!(t.priority, Priority::Five)).count(), DEEP_PINK),
        ("★★★★☆ High   ".to_string(), tasks.iter().filter(|t| matches!(t.priority, Priority::Four)).count(), Color::LightRed),
        ("★★★☆☆ Medium ".to_string(), tasks.iter().filter(|t| matches!(t.priority, Priority::Three)).count(), PINK),
        ("★★☆☆☆ Low    ".to_string(), tasks.iter().filter(|t| matches!(t.priority, Priority::Two)).count(), SOFT),
        ("★☆☆☆☆ Minimal".to_string(), tasks.iter().filter(|t| matches!(t.priority, Priority::One)).count(), MUTED),
    ];

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(chunks[1]);

    // ── Key figures ──
    let stats_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Total        ", Style::default().fg(MUTED)),
            Span::styled(format!("{}", total), Style::default().fg(PINK).add_modifier(Modifier::BOLD)),
            Span::styled(" task(s)", Style::default().fg(MUTED)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Urgent      ", Style::default().fg(MUTED)),
            Span::styled(format!("{}", urgentes), Style::default().fg(DEEP_PINK).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Categories  ", Style::default().fg(MUTED)),
            Span::styled(format!("{}", categories.len()), Style::default().fg(SOFT).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
Line::from(vec![
    Span::styled("  Progress     ", Style::default().fg(MUTED)),
    Span::styled(format!("{:.0}%", _progression), Style::default().fg(DEEP_PINK).add_modifier(Modifier::BOLD)),
]),
    ];

    let stats_widget = Paragraph::new(stats_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(SOFT))
                .title(Span::styled(" Summary ", Style::default().fg(PINK))),
        );
    f.render_widget(stats_widget, content_chunks[0]);

    // ── Priority bars ──
    let bar_width = content_chunks[1].width.saturating_sub(6) as usize;
    let mut bar_lines = vec![Line::from("")];

    for (label, count, color) in &prio_counts {
        let filled = if total > 0 { (count * bar_width) / total } else { 0 };
        let bar = "█".repeat(filled) + &"░".repeat(bar_width - filled);
        bar_lines.push(Line::from(vec![
            Span::styled(format!("  {} ", label), Style::default().fg(*color)),
            Span::styled(format!("{:2}", count), Style::default().fg(WHITE).add_modifier(Modifier::BOLD)),
        ]));
        bar_lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(bar, Style::default().fg(*color)),
        ]));
        bar_lines.push(Line::from(""));
    }

    let bars_widget = Paragraph::new(bar_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(SOFT))
                .title(Span::styled(" Breakdown by priority ", Style::default().fg(PINK))),
        );
    f.render_widget(bars_widget, content_chunks[1]);

    let footer = Paragraph::new(Line::from(Span::styled(
        "  esc back",
        Style::default().fg(MUTED),
    )))
    .block(Block::default().borders(Borders::TOP).border_style(Style::default().fg(SOFT)));
    f.render_widget(footer, chunks[2]);
}

// ─── UTILITY FUNCTIONS ───────────────────────────────────────────────────────
#[allow(dead_code)]
fn read_field(prompt: &str, default: Option<&str>) -> String {
    print!("\x1b[35m{}\x1b[0m : ", prompt);
    io::stdout().flush().unwrap();
    let mut val = String::new();
    io::stdin().read_line(&mut val).unwrap();
    let val = val.trim().to_string();
    if val.is_empty() { default.unwrap_or("").to_string() } else { val }
}

fn read_priority() -> Priority {
    println!("  \x1b[90mPriority:\x1b[0m");
    println!("  \x1b[31m5\x1b[0m Urgent  \x1b[33m4\x1b[0m High  \x1b[35m3\x1b[0m Medium  \x1b[90m2\x1b[0m Low  \x1b[90m1\x1b[0m Minimal");
    let val = read_field("  Your choice", Some("3"));
    parse_priority(&val)
}
fn parse_priority(s: &str) -> Priority {
    match s.trim() {
        "5" => Priority::Five,
        "4" => Priority::Four,
        "3" => Priority::Three,
        "2" => Priority::Two,
        "1" => Priority::One,
        _ => {
            println!("  \x1b[90mInvalid priority → using default priority 3\x1b[0m");  //test securitaire
            Priority::Three
        }
    }
}

fn read_deadline() -> NaiveDateTime {
    let s = read_field("  Deadline (DD/MM/YYYY HH:MM)", Some("31/12/2026 23:59"));
    NaiveDateTime::parse_from_str(&s, "%d/%m/%Y %H:%M")
        .unwrap_or_else(|_| {
            println!("  \x1b[90mInvalid format → using default date\x1b[0m");
            NaiveDateTime::parse_from_str("31/12/2026 23:59", "%d/%m/%Y %H:%M").unwrap()
        })
}

fn restore_tui() {
    enable_raw_mode().ok();
    execute!(io::stdout(), EnterAlternateScreen).ok();
}

// ─── BACKWARD-COMPATIBLE WRAPPERS ────────────────────────────────────────────
// These wrappers allow main.rs to keep calling the old signatures
// without breaking anything.
#[allow(dead_code)]
pub fn display_task_list(tasks: &[Task]) {
    // Text fallback if ratatui is not available
    println!("  \x1b[35m✦ MY TASK LIST\x1b[0m");
    if tasks.is_empty() {
        println!("  (No tasks)");
        return;
    }
    for (i, t) in tasks.iter().enumerate() {
        let statut = if t.completed { "✓ Terminée" } else { "À faire" };
        println!("  {}. \x1b[35m[{:?}]\x1b[0m {} - {}", i + 1, t.priority, t.name, statut);
        println!("     {}", t.description);
        println!("     📅 {}  |  {}", t.deadline.format("%d/%m/%Y"), t.categories.join(", "));
        println!();
    }
}
#[allow(dead_code)]
pub fn prompt_new_task() -> Task {
    prompt_new_task_tui().unwrap_or_else(|| Task {
        name: "Untitled".to_string(),
        description: String::new(),
        duration: Duration::minutes(30),
        priority: Priority::Three,
        deadline: NaiveDateTime::parse_from_str("2026-12-31 23:59", "%Y-%m-%d %H:%M").unwrap(),
        categories: vec![],
        completed: false,
    })
}
#[allow(dead_code)]
pub fn prompt_delete_task(max: usize) -> Option<usize> {
    if max == 0 {
        println!("Aucune tâche à supprimer.");
        return None;
    }

    print!("  \x1b[35mIndex to delete (1-{})\x1b[0m : ", max);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<usize>() {
        Ok(i) if i >= 1 && i <= max => Some(i - 1),
        Ok(_) => {
            println!("Index hors limites.");
            None
        }
        Err(_) => {
            println!("Veuillez entrer un nombre valide.");
            None
        }
    }
}
