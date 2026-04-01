use chrono::{ NaiveDateTime, Duration};
//NaiveDateTime, it's used to have a date and time without worrying about the time zone
//Duration, used to have a duration of, say, 30 minutes, 1 hour


///Main structure to manage tasks and events
#[derive(Debug, PartialEq)]
pub struct Manager
{
    pub tasks: Vec<Task>,
    pub events: Vec<Event>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Priority
{
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Event
{
    pub name: String,
    pub description: String,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub priority: Priority,
    pub categories: Vec<String>,
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
        if event.end > event.start
        {
            self.events.push(event);
            Ok(())
        }
        else
        {
            Err("The event must end after it has started.".to_string())
        }
    }
    #[allow(dead_code)]
    pub fn display_tasks(&self)
    {
        for (i, task) in self.tasks.iter().enumerate()
        {
            println!("{}: {} (Priority: {:?}) - Due: {}", i+1, task.name, task.priority, task.deadline);
        }
    }

    ///Returns tasks that are due in the next hour
    pub fn get_upcoming_tasks(&self, now: NaiveDateTime) -> Vec<&Task>
    {
        let one_hour_late = now + Duration::hours(1);
        self.tasks
            .iter()
            .filter(|t| t.deadline > now && t.deadline <= one_hour_late)
            .collect()
    }
}

