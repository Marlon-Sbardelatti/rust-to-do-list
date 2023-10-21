use std::io;
use std::path::PathBuf;
use crate::handles::handles::{handle_add, handle_done, handle_rm, handle_show, handle_not_done};

pub fn run(file_path_tasks: &PathBuf, file_path_marks: &PathBuf) {
    loop {
        println!("\n1 - Add a task");
        println!("2 - Remove a task");
        println!("3 - Show tasks");
        println!("4 - Mark task as done");
        println!("5 - Mark task as not done");
        println!("6 - Exit\n");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read user inpur");
        let choice: usize = choice.trim().parse().expect("Please type a number!");
        match choice {
            1 => {
                handle_add(file_path_tasks, file_path_marks);
            }
            2 => {
                handle_rm(file_path_tasks, file_path_marks);
            }
            3 => {
                handle_show(file_path_tasks, file_path_marks);
            }
            4 => {
                handle_done(file_path_marks);
            }
            5 => {
                handle_not_done(file_path_marks);
            }
            6 => {
                break;
            }
            _ => println!("Invalid number operation."),
        }
    }
}

