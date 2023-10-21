# Rust CLI To-Do List

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

A simple command-line tool to manage your to-do list. This Rust CLI application allows you to perform various operations on your tasks, such as adding, removing, marking as done, and more.

## Features

- Add a new task to your to-do list.
- Remove a task from your to-do list.
- Display your to-do list.
- Mark tasks as done.
- Mark tasks as not done.
- Command-line interface for easy use.

## Prerequisites

- Rust: Install Rust on your system if you haven't already. You can download it [here](https://www.rust-lang.org/learn/get-started).

## Installation

1. Clone this repository to your local machine.
   ```bash
   git clone https://github.com/yourusername/todo-list.git
   cd todo-list

## Build

1 - Build the project using the following command:

cargo build --release

2 - Copy the executable to a directory in your system's PATH (e.g., /usr/local/bin/) to use the tool from anywhere in the command line.

cp target/release/todo-list /usr/local/bin/

or

sudo cp target/release/todo-list /usr/local/bin/

## Usage

Run the application by executing the todo-list command in your terminal.

todo-list

## Operations

- 1 - Add a task: Add a new task to your to-do list.
- 2 - Remove a task: Remove a task from your to-do list.
- 3 - Show tasks: Display your to-do list.
- 4 - Mark task as done: Mark a task as done.
- 5 - Mark task as not done: Mark a task as not done.
- 6 - Exit: Quit the application.

## Configuration
The application creates and uses two files (marks.txt and todo-list.txt) to store your tasks and marks. These files act as a simple database for your tasks.

## License
This project is licensed under the MIT License - see the LICENSE file for details.

## Contributors
Marlon Sbardelatti(@Marlon-Sbardelatti)

## Acknowledgments
Just a simple project to improve my rust skills.
