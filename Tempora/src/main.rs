mod models;
mod logic;
mod filters;
mod ui;
use std::io::Write;
use chrono::{Duration, NaiveDateTime};
use models::{Manager, Task, Priority};
use logic::sort_tasks;
use filters::filter_by_category;

/*fn main() {

    println!("Testing Emma's and Lisa's implementations\n");

    // Create the main manager (Emma's structure)
    let mut manager = Manager::new();

    // Add tasks (Emma's add_task function)
    manager.add_task(Task {
        name: "Write report".to_string(),
        description: "Finish the final report".to_string(),
        duration: Duration::minutes(120),
        priority: Priority::Three,
        deadline: NaiveDateTime::parse_from_str("2026-03-20 18:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        categories: vec!["Work".to_string()],
    });

    manager.add_task(Task {
        name: "Prepare presentation".to_string(),
        description: "Create slides".to_string(),
        duration: Duration::minutes(60),
        priority: Priority::Five,
        deadline: NaiveDateTime::parse_from_str("2026-03-15 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        categories: vec!["School".to_string(), "Work".to_string()],
    });

    manager.add_task(Task {
        name: "Study Rust".to_string(),
        description: "Practice iterators and closures".to_string(),
        duration: Duration::minutes(90),
        priority: Priority::Four,
        deadline: NaiveDateTime::parse_from_str("2026-03-18 14:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        categories: vec!["School".to_string()],
    });

    println!("Tasks before sorting:");
    manager.display_tasks();

    // Lisa's sorting algorithm
    sort_tasks(&mut manager.tasks);

    println!("\nTasks after sorting (Lisa's algorithm):");
    manager.display_tasks();

    // Lisa's filtering by category
    let work_tasks = filter_by_category(&manager.tasks, "Work");

    println!("\nTasks in category 'Work' (Lisa's filter):");
    for task in work_tasks {
        println!("{} - {:?}", task.name, task.priority);
    }

    println!("\nTest completed.");

}*/

fn main() {
    let mut manager = Manager::new();

    /*tâche par défaut pour tester le tri 
    manager.add_task(Task {
        name: "Test Lisa".to_string(),
        description: "Vérifier le tri".to_string(),
        duration: Duration::minutes(30),
        priority: Priority::One,
        deadline: NaiveDateTime::parse_from_str("2026-03-20 18:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        categories: vec!["Test".to_string()],
    });*/

    println!("TEMPORA");

    loop {
        println!("\nMenu :");
        println!("1. Ajouter une tâche"); //tracy
        println!("2. Supprimer une tâche");
        println!("3. Modifier une tâche");
        println!("4. Afficher la liste triée");
        println!("5. Filtrer par catégorie");
        println!("6. Quitter");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Erreur de lecture");

        match choice.trim() {
            "1" => {
                //fct de saisie
                let new_task = ui::prompt_new_task();
                manager.add_task(new_task);
                println!("Tâche ajoutée avec succès !");
            }

            "2" => { //supp
                ui::display_task_list(&manager.tasks);
                if let Some(index) = ui::prompt_delete_task(manager.tasks.len()) {
                    if index < manager.tasks.len() {
                        manager.tasks.remove(index);
                        println!("Tâche supprimée !");
                    }
                }
            }


            "3" => {
                ui::display_task_list(&manager.tasks);
                print!("Index de la tâche à modifier : ");
                std::io::stdout().flush().unwrap();
                let mut idx_str = String::new();
                std::io::stdin().read_line(&mut idx_str).unwrap();
        
                if let Ok(idx) = idx_str.trim().parse::<usize>() {
                    if let Some(tache) = manager.tasks.get_mut(idx - 1) {
                        ui::prompt_modify_task(tache);
                        println!("Tâche mise à jour !");
                    }
                }
            }
            
            "4" => {
                //algo lisa
                sort_tasks(&mut manager.tasks);
                //list view
                ui::display_task_list(&manager.tasks);
            }
            "5" => {
                println!("Entrez la catégorie à filtrer :");
                let mut cat = String::new();
                std::io::stdin().read_line(&mut cat).expect("Erreur de lecture");
                let resultats = filter_by_category(&manager.tasks, cat.trim());
                println!("\n--- Résultats du filtre ---");
                for t in resultats {
                    println!("- {} [{:?}]", t.name, t.priority);
                }
            }
            "6" => {
                println!("See you soon !");
                break; 
            }
            _ => println!("Choix invalide, réessaie."),
        }
    }
}