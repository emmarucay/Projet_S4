use chrono::{ NaiveDateTime, Duration};
//NaiveDateTime, it's used to have a date and time without worrying about the time zone
//Duration, used to have a duration of, say, 30 minutes, 1 hour
use serde:: {Serialize, Deserialize};

///Main structure to manage tasks and events
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Manager
{
    pub tasks: Vec<Task>,
    pub events: Vec<Event>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Priority
{
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Task
{
    pub name: String,
    pub description: String,
    pub duration: Duration,
    pub priority : Priority,
    pub deadline: NaiveDateTime,
    pub categories: Vec<String>,
    pub completed: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Event
{
    pub name: String,
    pub description: String,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub priority: Priority,
    pub categories: Vec<String>,
}

impl Event {
    ///Checks whether two events overlap in time
    pub fn conflicts_with(&self, other: &Event) -> bool {
        self.start < other.end && self.end > other.start
    }
}

impl Manager
{
    pub fn new() -> Self
    {
        Self
        {
            tasks: Vec::new(),
            events: Vec::new(),
        }
    }
    ///Adds a new task to manager
    pub fn add_task(&mut self, task: Task)
    {
        self.tasks.push(task);
    }
    #[allow(dead_code)]
    ///Adds a new event to manager
    pub fn add_event(&mut self, event: Event) -> Result<(), String>
    {
        // Check that the event is valid
        if event.end <= event.start 
        {
            return Err("The event must end after it has started.".to_string());
        }

        // Check if the event overlaps with an existing one
        if let Some(conflict) = self.events.iter().find(|e| e.conflicts_with(&event)) 
        {
            return Err(format!(
                "Conflict with event '{}' ({} -> {})",
                conflict.name, conflict.start, conflict.end
            ));
        }

        self.events.push(event);
        Ok(())
    }
    #[allow(dead_code)]
    pub fn display_tasks(&self)
    {
        for (i, task) in self.tasks.iter().enumerate()
        {
            println!("{}: {} (Priority: {:?}) - Due: {}", i+1, task.name, task.priority, task.deadline);
        }
    }

    /*///Returns tasks that are due in the next hour
    pub fn get_upcoming_tasks(&self, now: NaiveDateTime) -> Vec<&Task>
    {
        let one_hour_late = now + Duration::hours(1);
        self.tasks
            .iter()
            .filter(|t| t.deadline > now && t.deadline <= one_hour_late)
            .collect()
    }*/
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    fn dt(s: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M").unwrap()
    }

    #[test]
    fn test_event_conflict() {
        let mut manager = Manager::new();

        let e1 = Event {
            name: "Cours".to_string(),
            description: "".to_string(),
            start: dt("2026-04-07 14:00"),
            end: dt("2026-04-07 15:00"),
            priority: Priority::Three,
            categories: vec![],
        };

        let e2 = Event {
            name: "Médecin".to_string(),
            description: "".to_string(),
            start: dt("2026-04-07 14:30"),
            end: dt("2026-04-07 15:30"),
            priority: Priority::Three,
            categories: vec![],
        };

        assert!(manager.add_event(e1).is_ok());
        assert!(manager.add_event(e2).is_err()); // DOIT échouer
    }
}