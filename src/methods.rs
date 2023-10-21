pub mod methods {
    use std::fs::{File, OpenOptions};
    use std::io::{self, BufRead, Write};
    use std::path::PathBuf;

    pub fn add_task(
        file_path_tasks: &PathBuf,
        file_path_marks: &PathBuf,
        text: &str,
    ) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path_tasks)?;

        file.write_all(text.as_bytes())?;

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path_marks)?;
        let mark = "f\n";
        file.write_all(mark.as_bytes())?;

        Ok(())
    }

    pub fn remove_task(
        file_path_tasks: &PathBuf,
        file_path_marks: &PathBuf,
        input_num: usize,
    ) -> io::Result<()> {
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
            line_to_remove = input_num - 1;
        } else {
            line_to_remove = input_num;
        }

        if line_to_remove < lines.len() {
            lines.remove(line_to_remove);
            marks.remove(line_to_remove);

            let mut file = File::create(&file_path_tasks)?;
            for line in lines {
                writeln!(file, "{}", line)?;
            }

            let mut file2 = File::create(&file_path_marks)?;
            for mark in marks {
                writeln!(file2, "{}", mark)?;
            }
        } else {
            println!("Task number is out of range.");
        }

        Ok(())
    }

    pub fn show_tasks(file_path_tasks: &PathBuf, file_path_marks: &PathBuf) -> io::Result<()> {
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

    pub fn mark_done(file_path_marks: &PathBuf, input_num: usize) -> io::Result<()> {
        let mut marks: Vec<String> = {
            let file = File::open(&file_path_marks)?;
            let reader = io::BufReader::new(file);
            reader.lines().filter_map(|line| line.ok()).collect()
        };

        let line_to_remove;
        if input_num != 0 {
            line_to_remove = input_num - 1;
        } else {
            line_to_remove = input_num;
        }

        if line_to_remove < marks.len() {
            marks[line_to_remove] = "t".to_string();

            let mut file = File::create(&file_path_marks)?;
            for line in marks {
                writeln!(file, "{}", line)?;
            }
        } else {
            println!("Task number is out of range.");
        }
        Ok(())
    }

    pub fn mark_not_done(file_path_marks: &PathBuf, input_num: usize) -> io::Result<()> {
        let mut marks: Vec<String> = {
            let file = File::open(&file_path_marks)?;
            let reader = io::BufReader::new(file);
            reader.lines().filter_map(|line| line.ok()).collect()
        };

        let line_to_remove;
        if input_num != 0 {
            line_to_remove = input_num - 1;
        } else {
            line_to_remove = input_num;
        }

        if line_to_remove < marks.len() {
            marks[line_to_remove] = "f".to_string();

            let mut file = File::create(&file_path_marks)?;
            for line in marks {
                writeln!(file, "{}", line)?;
            }
        } else {
            println!("Task number is out of range.");
        }

        Ok(())
    }
}
