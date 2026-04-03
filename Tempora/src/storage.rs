use std::fs::File;
use std::io::{Read, Write};
use crate::models::Manager;

pub fn save_to_file(manager: &Manager, filename: &str) -> Result<(), String>
{
    let json = serde_json::to_string_pretty(manager)
        .map_err(|e| e.to_string())?; //allows you to translate all types of errors for the Result

    let mut file = File::create(filename).map_err(|e| e.to_string())?;
    file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

//Loads date from a JSON file to restore the manager's state (tasts and events) at startup
pub fn load_from_file(filename: &str) -> Result<Manager, String> 
{
    let mut file = File::open(filename).map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| e.to_string())?;

    let manager: Manager = serde_json::from_str(&contents)
        .map_err(|e| e.to_string())?;
    Ok(manager)
}
