use crate::models::Task;

// Keyword search (name, description, categories)
pub fn search_tasks<'a>(tasks: &'a [Task], keyword: &str) -> Vec<&'a Task> {
    let keyword = keyword.trim().to_lowercase();

    if keyword.is_empty() {
        return Vec::new();
    }

    tasks
        .iter()
        .filter(|task| {
            task.name.to_lowercase().contains(&keyword)
                || task.description.to_lowercase().contains(&keyword)
                || task.categories
                    .iter()
                    .any(|cat| cat.to_lowercase().contains(&keyword))
        })
        .collect()
}
