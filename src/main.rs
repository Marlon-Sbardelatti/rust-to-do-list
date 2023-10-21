use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

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
    // Define the path to the file in your home directory
    let home_dir = dirs::home_dir(); // This requires the `dirs` crate; make sure to add it to your `Cargo.toml`
    if let Some(home_dir) = home_dir {
        let file_path_tasks = home_dir.join("todo-list.txt");
        let file_path_marks = home_dir.join("marks.txt");
        if file_path_tasks.exists() && file_path_marks.exists() {
            println!("File already exists.");
            run(&file_path_tasks, &file_path_marks);
        } else {
            // The file doesn't exist; create it
            match File::create(&file_path_tasks) {
                Ok(_) => {
                    println!("File created successfully.");
                }
                Err(err) => {
                    println!("Failed to create the file: {}", err);
                }
            }
            match File::create(&file_path_marks) {
                Ok(_) => {
                    println!("File created successfully.");
                }
                Err(err) => {
                    println!("Failed to create the file: {}", err);
                }
            }
            run(&file_path_tasks, &file_path_marks);
        }
    } else {
        println!("Failed to determine the home directory.");
    }
}
fn run(file_path_tasks: &PathBuf, file_path_marks: &PathBuf) {
    loop {
        println!("\n1 - Add a Task");
        println!("2 - Remove a Task");
        println!("3 - Show Tasks");
        println!("4 - Mark Task as done");
        println!("5 - Exit\n");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read user inpur");
        let choice: usize = choice.trim().parse().expect("Please type a number!");
        match choice {
            1 => {
                println!("Add your task: ");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read user input");

                let text_new_line = format!("{}", input);
                add_task(file_path_tasks, file_path_marks, &text_new_line);
            }
            2 => {
                println!("Type the task number to delete it:");
                let mut input_num = String::new();
                io::stdin()
                    .read_line(&mut input_num)
                    .expect("Failed to read user input");

                let input_num: usize = input_num.trim().parse().expect("Please type a number!");
                remove_task(file_path_tasks, file_path_marks, input_num);
            }
            3 => {
                println!("Your tasks: ");
                show_tasks(file_path_tasks, file_path_marks);
            }
            4 => {
                println!("Type the task number to mark as done:");
                let mut input_num = String::new();
                io::stdin()
                    .read_line(&mut input_num)
                    .expect("Failed to read user input");

                let input_num: usize = input_num.trim().parse().expect("Please type a number!");
                mark_done(file_path_marks, input_num);
            }
            5 => {
                break;
            }
            _ => println!("Invalid number operation."),
        }
    }
}
fn add_task(
    file_path_tasks: &PathBuf,
    file_path_marks: &PathBuf,
    text: &str,
) -> std::io::Result<()> {
    // Open the file in append mode
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path_tasks)?;

    // Text you want to append to the file

    // Write the text to the file
    file.write_all(text.as_bytes())?;

    println!("Text appended to the file.");

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path_marks)?;
    let mark = "f\n";
    file.write_all(mark.as_bytes())?;

    Ok(())
}

fn remove_task(
    file_path_tasks: &PathBuf,
    file_path_marks: &PathBuf,
    input_num: usize,
) -> io::Result<()> {
    // Read the file into memory, excluding the line you want to remove
    let mut lines: Vec<String> = {
        let file = File::open(&file_path_tasks)?;
        let reader = io::BufReader::new(file);
        reader.lines().filter_map(|line| line.ok()).collect()
    };
    let mut marks: Vec<String> = {
        let file = File::open(&file_path_marks)?;
        let reader = io::BufReader::new(file);
        reader.lines().filter_map(|line| line.ok()).collect()
    };

    let line_to_remove;
    if input_num != 0 {
        line_to_remove = input_num - 1; // Replace with the desired line number
    } else {
        line_to_remove = input_num; // Replace with the desired line number
    }
    // Specify the line number you want to remove (0-indexed)

    if line_to_remove < lines.len() {
        // Remove the specified line
        lines.remove(line_to_remove);
        marks.remove(line_to_remove);
        // Write the modified content back to the file
        let mut file = File::create(&file_path_tasks)?;
        for line in lines {
            writeln!(file, "{}", line)?;
        }
        let mut file2 = File::create(&file_path_marks)?;
        for mark in marks {
            writeln!(file2, "{}", mark)?;
        }

        println!("Line removed and file updated.");
    } else {
        println!("Line number is out of range.");
    }

    Ok(())
}

fn show_tasks(file_path_tasks: &PathBuf, file_path_marks: &PathBuf) -> io::Result<()> {
    let marks: Vec<String> = {
        let file = File::open(&file_path_marks)?;
        let reader = io::BufReader::new(file);
        reader.lines().filter_map(|line| line.ok()).collect()
    };

    let file = File::open(file_path_tasks)?;

    let reader = io::BufReader::new(file);

    let mut idx = 1;
    let mut idx_mark = 0;
    for line in reader.lines() {
        match line {
            Ok(content) => {
                let mut mark = "❌";
                if marks[idx_mark] == "t" {
                    mark = "✅";
                }
                println!("{} - {} {}", idx, content, mark);
                idx = idx + 1;
                idx_mark = idx_mark + 1;
            }
            Err(e) => {
                eprintln!("Error reading a line: {}", e);
            }
        }
    }

    Ok(())
}

fn mark_done(file_path_marks: &PathBuf, input_num: usize) -> io::Result<()> {
    let mut marks: Vec<String> = {
        let file = File::open(&file_path_marks)?;
        let reader = io::BufReader::new(file);
        reader.lines().filter_map(|line| line.ok()).collect()
    };

    let line_to_remove;
    if input_num != 0 {
        line_to_remove = input_num - 1; // Replace with the desired line number
    } else {
        line_to_remove = input_num; // Replace with the desired line number
    }

    if line_to_remove < marks.len() {
        // Remove the specified line
        marks[line_to_remove] = "t".to_string();

        // Write the modified content back to the file
        let mut file = File::create(&file_path_marks)?;
        for line in marks {
            writeln!(file, "{}", line)?;
        }
        println!("Line removed and file updated.");
    } else {
        println!("Line number is out of range.");
    }
    Ok(())
}

