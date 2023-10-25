pub mod args {
    use crate::arg_handles::arg_handles::{
        handle_add, handle_done, handle_not_done, handle_rm, handle_show,
    };
    use clap::{Arg, Command};
    use std::env;
    use std::path::PathBuf;

    pub fn handle_arg(file_path_tasks: &PathBuf, file_path_marks: &PathBuf) -> bool {
        let mut found = false;
        let params =
            Command::new("to-do list")
                .version("1.0.0")
                .about("A simple to-do list, use the commands below to add, remove, show, mark and unmark tasks")
                .author("Marlon Sbardelatti, @Marlon-Sbardelatti")
                .arg(
                    Arg::new("add")
                        .short('a')
                        .long("add")
                        .help("Add a task, use quotes for more than one word"),
                )
                .arg(
                    Arg::new("remove")
                        .short('r')
                        .long("remove")
                        .help("Remove a task, type the number of the line to remove"),
                )
                .arg(
                    Arg::new("show")
                        .short('s')
                        .long("show")
                        .num_args(0)
                        .help("Show the tasks")
                )
                .arg(
                    Arg::new("mark")
                        .short('m')
                        .long("mark")
                        .help("Mark a task as done, type the number of the line to mark as done"),
                )
                .arg(Arg::new("unmark").short('u').long("unmark").help(
                    "Mark a task as not done, type the number of the line to mark as not done",
                ))
                .get_matches();

        if let Some(text) = params.get_one::<String>("add") {
            handle_add(&file_path_tasks, &file_path_marks, text);
            found = true;
        } else if let Some(num) = params.get_one::<String>("remove") {
            match num.parse() {
                Ok(num) => {
                    handle_rm(&file_path_tasks, &file_path_marks, num);
                }
                Err(e) => println!("Error parsing the number. {}", e),
            }
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

        let show: Vec<String> = env::args().collect();
        let mut has_s = false;
        let mut count_flags = 0;
        for i in show {
            if i == "-s" || i == "--show" {
                has_s = true;
            }
            count_flags = count_flags + 1;
        }
        if has_s {
            handle_show(&file_path_tasks, &file_path_marks);
            found = true;
        } else {
            if count_flags > 1 {
                if params.contains_id("show") {
                    handle_show(&file_path_tasks, &file_path_marks);
                    found = true;
                }
            }
        }
        found
    }
}
