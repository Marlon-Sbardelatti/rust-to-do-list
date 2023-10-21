use std::fs::File;
mod run;
use crate::run::run;
mod methods;
mod handles;
fn main() {
    println!(
        "
    ████████╗░█████╗░░░░░░░██████╗░░█████╗░  ██╗░░░░░██╗░██████╗████████╗
    ╚══██╔══╝██╔══██╗░░░░░░██╔══██╗██╔══██╗  ██║░░░░░██║██╔════╝╚══██╔══╝
    ░░░██║░░░██║░░██║█████╗██║░░██║██║░░██║  ██║░░░░░██║╚█████╗░░░░██║░░░
    ░░░██║░░░██║░░██║╚════╝██║░░██║██║░░██║  ██║░░░░░██║░╚═══██╗░░░██║░░░
    ░░░██║░░░╚█████╔╝░░░░░░██████╔╝╚█████╔╝  ███████╗██║██████╔╝░░░██║░░░
    ░░░╚═╝░░░░╚════╝░░░░░░░╚═════╝░░╚════╝░  ╚══════╝╚═╝╚═════╝░░░░╚═╝░░░"
    );
    let home_dir = dirs::home_dir(); 
    if let Some(home_dir) = home_dir {
        let file_path_tasks = home_dir.join("todo-list.txt");
        let file_path_marks = home_dir.join("marks.txt");
        if file_path_tasks.exists() && file_path_marks.exists() {
            run(&file_path_tasks, &file_path_marks);
        } else {
            match File::create(&file_path_tasks) {
                Ok(_) => {}
                Err(err) => {
                    println!("Failed to create the tasks file: {}", err);
                }
            }
            match File::create(&file_path_marks) {
                Ok(_) => {}
                Err(err) => {
                    println!("Failed to create marks the file: {}", err);
                }
            }
            run(&file_path_tasks, &file_path_marks);
        }
    } else {
        println!("Failed to determine the home directory.");
    }
}
