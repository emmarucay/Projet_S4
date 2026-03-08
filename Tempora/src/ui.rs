
use std::io::{self, Write};
use crate::models::{Task, Priority};
use chrono::{Duration, NaiveDateTime};

//show the task's list as a text
pub fn display_task_list(tasks: &[Task]) {
    println!("       MA LISTE DE TÂCHES      ");

    
    if tasks.is_empty() {
        println!("(Aucune tâche pour le moment)");
    } else {
        for (i, task) in tasks.iter().enumerate() {
            println!("{}. [{:?}] {}", i + 1, task.priority, task.name);
            println!("   Description : {}", task.description);
            println!("   Deadline : {}", task.deadline);
            println!("   Catégories : {}", task.categories.join(", "));
            println!("   --");
        }
    }
}

//write each parameter one by one
pub fn prompt_new_task() -> Task {
    println!("\nNOUVELLE TÂCHE");

    //write the name
    print!("Nom : ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    //description
    print!("Description : ");
    io::stdout().flush().unwrap();
    let mut desc = String::new();
    io::stdin().read_line(&mut desc).unwrap();

    //priority
    print!("Priorité (1 à 5) : ");
    io::stdout().flush().unwrap();
    let mut prio_str = String::new();
    io::stdin().read_line(&mut prio_str).unwrap();
    
    let priority = match prio_str.trim() {
        "1" => Priority::One,
        "2" => Priority::Two,
        "3" => Priority::Three,
        "4" => Priority::Four,
        "5" => Priority::Five,
        _ => {
            println!("(Choix invalide, priorité 3 par défaut)");
            Priority::Three
        },
    };

    // category
    print!("Catégories (ex: Travail, Perso) : ");
    io::stdout().flush().unwrap();
    let mut cats_str = String::new();
    io::stdin().read_line(&mut cats_str).unwrap();
    let categories: Vec<String> = cats_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    //deadline
    println!("Deadline (format AAAA-MM-JJ HH:MM, ex: 2026-03-25 14:00) : ");
    io::stdout().flush().unwrap();
    let mut date_str = String::new();
    io::stdin().read_line(&mut date_str).unwrap();

    // On essaie de lire la date. Si l'utilisateur se trompe, on met une date par défaut
    // (C'est ici que le travail de Farah sur la gestion d'erreurs sera utile plus tard)
    let deadline = NaiveDateTime::parse_from_str(date_str.trim(), "%Y-%m-%d %H:%M")
        .unwrap_or_else(|_| {
            println!("Format invalide. Date par défaut : 2026-12-31 23:59");
            NaiveDateTime::parse_from_str("2026-12-31 23:59", "%Y-%m-%d %H:%M").unwrap()
        });

    Task {
        name: name.trim().to_string(),
        description: desc.trim().to_string(),
        duration: Duration::minutes(30),
        priority,
        deadline,
        categories,
    }
}


pub fn prompt_delete_task(max: usize) -> Option<usize> {
    if max == 0 { return None; }
    print!("Index de la tâche à supprimer (1-{}) : ", max);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().parse::<usize>().ok().map(|i| i - 1) 
}


pub fn prompt_modify_task(task: &mut Task) {
    println!("\n--- Modification de : {} ---", task.name);
    println!("(Appuyez sur Entrée pour conserver la valeur actuelle)");

    // 1. Modifier le Nom
    print!("Nouveau nom [{}] : ", task.name);
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    if !name.trim().is_empty() { 
        task.name = name.trim().to_string(); 
    }

    // 2. Modifier la Description
    print!("Nouvelle description [{}] : ", task.description);
    io::stdout().flush().unwrap();
    let mut desc = String::new();
    io::stdin().read_line(&mut desc).unwrap();
    if !desc.trim().is_empty() { 
        task.description = desc.trim().to_string(); 
    }

    // 3. Modifier la Priorité
    print!("Nouvelle priorité (1-5) [actuelle: {:?}] : ", task.priority);
    io::stdout().flush().unwrap();
    let mut prio_str = String::new();
    io::stdin().read_line(&mut prio_str).unwrap();
    if !prio_str.trim().is_empty() {
        task.priority = match prio_str.trim() {
            "1" => Priority::One,
            "2" => Priority::Two,
            "3" => Priority::Three,
            "4" => Priority::Four,
            "5" => Priority::Five,
            _ => {
                println!("Choix invalide, on garde l'ancienne priorité.");
                task.priority.clone()
            },
        };
    }

    // 4. Modifier les Catégories
    let current_cats = task.categories.join(", ");
    print!("Nouvelles catégories (séparées par des virgules) [{}] : ", current_cats);
    io::stdout().flush().unwrap();
    let mut cats_str = String::new();
    io::stdin().read_line(&mut cats_str).unwrap();
    if !cats_str.trim().is_empty() {
        task.categories = cats_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
    }
    
    println!("\n✅ Modification terminée !");
}