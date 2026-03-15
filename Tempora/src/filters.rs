use crate::models::Task;

// Returns only the tasks that contain the requested category
// A task may have multiple categories, so we check inside the Vec<String>
pub fn filter_by_category<'a>(tasks: &'a [Task], wanted_category: &str) -> Vec<&'a Task> {
    tasks.iter()
        .filter(|task| {
            task.categories
                .iter()
                .any(|category| category.eq_ignore_ascii_case(wanted_category))
        })
        .collect()
}

// Generic filtering function using a closure
// This allows filtering tasks using any condition
pub fn filter_tasks<F>(tasks: &[Task], predicate: F) -> Vec<&Task>
where
    F: Fn(&Task) -> bool,
{
    tasks.iter()
        .filter(|task| predicate(task))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Priority, Task};
    use chrono::{Duration, NaiveDateTime};

    #[test]
    fn test_filter_by_category_work() {
        let tasks = vec![
            Task {
                name: "Rapport".to_string(),
                description: "Finir le rapport".to_string(),
                duration: Duration::minutes(60),
                priority: Priority::Three,
                deadline: NaiveDateTime::parse_from_str("2026-03-20 18:00", "%Y-%m-%d %H:%M").unwrap(),
                categories: vec!["Work".to_string(), "School".to_string()],
                completed: false,
            },
            Task {
                name: "Courses".to_string(),
                description: "Acheter du lait".to_string(),
                duration: Duration::minutes(20),
                priority: Priority::One,
                deadline: NaiveDateTime::parse_from_str("2026-03-21 10:00", "%Y-%m-%d %H:%M").unwrap(),
                categories: vec!["Perso".to_string()],
                completed: false,
            },
        ];

        let result = filter_by_category(&tasks, "Work");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "Rapport");
    }
}