


use crate::models::Task;

#[allow(dead_code)]
pub fn calculate_progress(tasks: &[Task]) -> f32 {//calculates the pourcentage of tasks that have
                                                  //been done by dividing them by the total of
                                                  //tasks 
    let total = tasks.len();

    if total == 0 {
        return 0.0;
    }

    let completed = tasks.iter()
        .filter(|t| t.completed)
        .count();

    (completed as f32 / total as f32) * 100.0
}


#[cfg(test)]
mod tests {//different tests
    use super::*;
    use crate::models::{Task, Priority};
    use chrono::{Duration, NaiveDateTime};

    #[test]
    fn test_progression_vide() {// test the progration of 0%
        let tasks: Vec<Task> = vec![];
        let p = calculate_progress(&tasks);
        assert_eq!(p, 0.0);//wuch should give us this
    }

    #[test]
    fn test_progression_moitie() {// this one test the progression of 50%
        let tasks = vec![
            Task {
                name: "A".to_string(),// it creates a structure
                description: "".to_string(),
                duration: Duration::minutes(30),
                priority: Priority::One,
                deadline: NaiveDateTime::parse_from_str("2026-03-25 14:00", "%Y-%m-%d %H:%M").unwrap(),
                categories: vec!["Test".to_string()],
                completed: true,
            },
            Task {
                name: "B".to_string(),
                description: "".to_string(),
                duration: Duration::minutes(30),
                priority: Priority::Two,
                deadline: NaiveDateTime::parse_from_str("2026-03-26 14:00", "%Y-%m-%d %H:%M").unwrap(),
                categories: vec!["Test".to_string()],
                completed: false,
            },
        ];

        let p = calculate_progress(&tasks);// and calls the function
        assert_eq!(p, 50.0);
    }
}
