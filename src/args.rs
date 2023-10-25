pub mod args {
    use crate::arg_handles::arg_handles::{
        handle_add, handle_done, handle_not_done, handle_rm, handle_show,
    };
    use clap::{command, Arg};
    use std::path::PathBuf;

    pub fn handle_arg(file_path_tasks: &PathBuf, file_path_marks: &PathBuf) -> bool {
        let mut found = false;
        let params = command!()
            .arg(Arg::new("add").short('a').long("add"))
            .arg(Arg::new("remove").short('r').long("remove"))
            .arg(Arg::new("show").short('s').long("show"))
            .arg(Arg::new("mark").short('m').long("mark"))
            .arg(Arg::new("unmark").short('u').long("unmark"))
            .get_matches();

        if let Some(_text) = params.get_one::<String>("add") {
            handle_add(&file_path_tasks, &file_path_marks);
            found = true;
        } else if let Some(num) = params.get_one::<String>("remove") {
            match num.parse() {
                Ok(num) => {
                    handle_rm(&file_path_tasks, &file_path_marks, num);
                }
                Err(e) => println!("Error parsing the number. {}", e),
            }
            found = true;
        } else if let Some(_text) = params.get_one::<String>("show") {
            handle_show(&file_path_tasks, &file_path_marks);
            found = true;
        } else if let Some(num) = params.get_one::<String>("mark") {
            match num.parse() {
                Ok(num) => handle_done(&file_path_marks, num),
                Err(e) => println!("Error parsing the number. {}", e),
            }
            found = true;
        } else if let Some(num) = params.get_one::<String>("unmark") {
            match num.parse() {
                Ok(num) => handle_not_done(&file_path_marks, num),
                Err(e) => println!("Error parsing the number. {}", e),
            }
            found = true;
        }
        found
    }
}
