use crate::models::{Priority, Task};

// Converts the Priority enum into a numeric value
// This is useful to compare priorities when sorting tasks
fn priority_value(priority: &Priority) -> u8 {
    match priority {
        Priority::One => 1,
        Priority::Two => 2,
        Priority::Three => 3,
        Priority::Four => 4,
        Priority::Five => 5,
    }
}

// Sorts tasks using multiple criteria:
// 1. Higher priority first
// 2. If priorities are equal, earliest deadline first
// 3. If still equal, alphabetical order by name
pub fn sort_tasks(tasks: &mut [Task]) {
    tasks.sort_by(|a, b| {
        priority_value(&b.priority)
            .cmp(&priority_value(&a.priority))
            .then_with(|| a.deadline.cmp(&b.deadline))
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
}