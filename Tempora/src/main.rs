mod models;
mod logic;
mod filters;

use chrono::{Duration, NaiveDateTime};
use models::{Manager, Task, Priority};
use logic::sort_tasks;
use filters::filter_by_category;

fn main() {

    println!("Testing Emma's and Lisa's implementations\n");

    // Create the main manager (Emma's structure)
    let mut manager = Manager::new();

    // Add tasks (Emma's add_task function)
    manager.add_task(Task {
        name: "Write report".to_string(),
        description: "Finish the final report".to_string(),
        duration: Duration::minutes(120),
        priority: Priority::Three,
        deadline: NaiveDateTime::parse_from_str("2026-03-20 18:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        categories: vec!["Work".to_string()],
    });

    manager.add_task(Task {
        name: "Prepare presentation".to_string(),
        description: "Create slides".to_string(),
        duration: Duration::minutes(60),
        priority: Priority::Five,
        deadline: NaiveDateTime::parse_from_str("2026-03-15 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        categories: vec!["School".to_string(), "Work".to_string()],
    });

    manager.add_task(Task {
        name: "Study Rust".to_string(),
        description: "Practice iterators and closures".to_string(),
        duration: Duration::minutes(90),
        priority: Priority::Four,
        deadline: NaiveDateTime::parse_from_str("2026-03-18 14:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        categories: vec!["School".to_string()],
    });

    println!("Tasks before sorting:");
    manager.display_tasks();

    // Lisa's sorting algorithm
    sort_tasks(&mut manager.tasks);

    println!("\nTasks after sorting (Lisa's algorithm):");
    manager.display_tasks();

    // Lisa's filtering by category
    let work_tasks = filter_by_category(&manager.tasks, "Work");

    println!("\nTasks in category 'Work' (Lisa's filter):");
    for task in work_tasks {
        println!("{} - {:?}", task.name, task.priority);
    }

    println!("\nTest completed.");
}