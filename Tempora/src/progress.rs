


use crate::models::Task;


pub fn calculate_progress(tasks: &[Task]) -> f32 {
    let total = tasks.len();

    if total == 0 {
        return 0.0;
    }

    let completed = tasks.iter()
        .filter(|t| t.completed)
        .count();

    (completed as f32 / total as f32) * 100.0
}
