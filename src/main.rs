// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.

use std::io::{self, Write};

// * Get user input
fn get_input() -> Option<String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Please enter your data again!");
    input = input.trim().to_owned();
    if input == "" {
        None
    } else {
        Some(input)
    }
}

fn main() {
    print!("Please input a number: ");
    io::stdout().flush().expect("Flushing failed");
    match get_input() {
        Some(input) => println!("{}", input),
        None => println!("{}", "Nothing was typed"),
    }
}
