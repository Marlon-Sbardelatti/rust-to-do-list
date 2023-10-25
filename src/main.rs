use std::fs::File;
mod run;
use crate::run::run;
mod arg_handles;
mod args;
mod handles;
mod methods;
use crate::args::args::handle_arg;
fn main() {
    // let home_dir = dirs::home_dir().unwrap();
    // let file_path_tasks = home_dir.join("todo-list.txt");
    // let file_path_marks = home_dir.join("marks.txt");

    // let params = command!().arg(Arg::new("add").short('a')).get_matches();
    // if let Some(text) = params.get_one::<String>("add") {
    //     let home_dir = dirs::home_dir().unwrap();
    //     let file_path_tasks = home_dir.join("todo-list.txt");
    //     let file_path_marks = home_dir.join("marks.txt");
    //     add_task(&file_path_tasks, &file_path_marks, text);
    // }
    let home_dir = dirs::home_dir();
    if let Some(home_dir) = home_dir {
        let file_path_tasks = home_dir.join("todo-list.txt");
        let file_path_marks = home_dir.join("marks.txt");
        if file_path_tasks.exists() && file_path_marks.exists() {
            let single_run = handle_arg(&file_path_tasks, &file_path_marks);
            if !single_run {
                println!(
                    "
    ████████╗░█████╗░░░░░░░██████╗░░█████╗░  ██╗░░░░░██╗░██████╗████████╗
    ╚══██╔══╝██╔══██╗░░░░░░██╔══██╗██╔══██╗  ██║░░░░░██║██╔════╝╚══██╔══╝
    ░░░██║░░░██║░░██║█████╗██║░░██║██║░░██║  ██║░░░░░██║╚█████╗░░░░██║░░░
    ░░░██║░░░██║░░██║╚════╝██║░░██║██║░░██║  ██║░░░░░██║░╚═══██╗░░░██║░░░
    ░░░██║░░░╚█████╔╝░░░░░░██████╔╝╚█████╔╝  ███████╗██║██████╔╝░░░██║░░░
    ░░░╚═╝░░░░╚════╝░░░░░░░╚═════╝░░╚════╝░  ╚══════╝╚═╝╚═════╝░░░░╚═╝░░░"
                );
                run(&file_path_tasks, &file_path_marks);
            }
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
            let single_run = handle_arg(&file_path_tasks, &file_path_marks);

            if !single_run {
                println!(
                    "
    ████████╗░█████╗░░░░░░░██████╗░░█████╗░  ██╗░░░░░██╗░██████╗████████╗
    ╚══██╔══╝██╔══██╗░░░░░░██╔══██╗██╔══██╗  ██║░░░░░██║██╔════╝╚══██╔══╝
    ░░░██║░░░██║░░██║█████╗██║░░██║██║░░██║  ██║░░░░░██║╚█████╗░░░░██║░░░
    ░░░██║░░░██║░░██║╚════╝██║░░██║██║░░██║  ██║░░░░░██║░╚═══██╗░░░██║░░░
    ░░░██║░░░╚█████╔╝░░░░░░██████╔╝╚█████╔╝  ███████╗██║██████╔╝░░░██║░░░
    ░░░╚═╝░░░░╚════╝░░░░░░░╚═════╝░░╚════╝░  ╚══════╝╚═╝╚═════╝░░░░╚═╝░░░"
                );
                run(&file_path_tasks, &file_path_marks);
            }
        }
    } else {
        println!("Failed to determine the home directory.");
    }
}
