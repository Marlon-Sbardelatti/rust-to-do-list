pub mod arg_handles {
    use crate::methods::methods::{add_task, mark_done, remove_task, show_tasks, mark_not_done};
    use std::path::PathBuf;

    pub fn handle_add(file_path_tasks: &PathBuf, file_path_marks: &PathBuf, input: &String) {
        // println!("Add your task: ");
        // let mut input = String::new();
        // io::stdin()
        //     .read_line(&mut input)
        //     .expect("Failed to read user input");
        let text_new_line = format!("{}\n", input);
        match add_task(file_path_tasks, file_path_marks, &text_new_line) {
            Ok(_) => {}
            Err(err) => {
                println!("Failed to add the task: {}", err);
            }
        }
    }

    pub fn handle_rm(file_path_tasks: &PathBuf, file_path_marks: &PathBuf, input_num: usize) {
        match remove_task(file_path_tasks, file_path_marks, input_num) {
            Ok(_) => {}
            Err(err) => {
                println!("Failed to remove the task: {}", err);
            }
        }
    }

    pub fn handle_show(file_path_tasks: &PathBuf, file_path_marks: &PathBuf) {
        println!("Your tasks: ");
        match show_tasks(file_path_tasks, file_path_marks) {
            Ok(_) => {}
            Err(err) => {
                println!("Failed to show the tasks: {}", err);
            }
        }
    }

    pub fn handle_done(file_path_marks: &PathBuf, input_num: usize) {
        match mark_done(file_path_marks, input_num) {
            Ok(_) => {}
            Err(err) => {
                println!("Failed to mark task as done: {}", err);
            }
        }
    }
    pub fn handle_not_done(file_path_marks: &PathBuf, input_num: usize) {
        match mark_not_done(file_path_marks, input_num) {
            Ok(_) => {}
            Err(err) => {
                println!("Failed to mark task as done: {}", err);
            }
        }
    }
}
