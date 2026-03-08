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