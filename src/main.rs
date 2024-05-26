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

use std::{
    collections::HashMap,
    io::{self, Write},
};

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

// * Get bill amount
fn get_bill_amount() -> Option<f64> {
    loop {
        print!("Amount: ");
        io::stdout().flush().expect("Flush failed");
        let input = get_input().unwrap_or_else(|| "".to_owned());
        if input == "" {
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(value) => return Some(value),
            Err(e) => println!("Please enter a float: {}", e),
        }
    }
}

enum MainMenu {
    AddBill,
    ViewBills,
    RemoveBill,
    UpdateBill,
}

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_owned(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        if let Some(bill) = self.inner.get_mut(name) {
            bill.amount = amount;
            true
        } else {
            false
        }
    }
}

mod menu {
    use crate::{get_bill_amount, get_input, Bill, Bills};
    use std::io::{self, Write};

    pub fn add_bill(bills: &mut Bills) {
        print!("Bill name: ");
        io::stdout().flush().expect("Flush failed");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(value) => value,
            None => return,
        };
        let bill = Bill { name, amount };
        bills.add(bill);
        println!("Bill added!");
    }

    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
    }

    pub fn remove_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
        print!("Enter bill name to remove: ");
        io::stdout().flush().expect("Flush failed");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        if bills.remove(&name) {
            println!("Bill removed");
        } else {
            println!("Bill not removed");
        }
    }

    pub fn update_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
        print!("Enter bill name to update: ");
        io::stdout().flush().expect("Flush failed");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(value) => value,
            None => return,
        };
        if bills.update(&name, amount) {
            println!("Bill updated");
        } else {
            println!("Bill not found");
        }
    }
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBills),
            "3" => Some(Self::RemoveBill),
            "4" => Some(Self::UpdateBill),
            _ => None,
        }
    }

    fn show() {
        println!("");
        println!("--Billing Application--");
        let menu = [
            "Add Bill".to_owned(),
            "View Bills".to_owned(),
            "Remove Bill".to_owned(),
            "Update Bill".to_owned(),
        ];
        for (index, item) in menu.iter().enumerate() {
            println!("{}. {}", index + 1, item);
        }
        println!("");
        print!("Enter selection: ");
        io::stdout().flush().expect("Flushing failed");
    }
}

fn run_program() -> Option<()> {
    use menu::*;
    // Initialization
    let mut bills = Bills::new();
    // User interaction
    loop {
        MainMenu::show();
        let input = get_input()?;
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => add_bill(&mut bills),
            Some(MainMenu::ViewBills) => view_bills(&bills),
            Some(MainMenu::RemoveBill) => remove_bill(&mut bills),
            Some(MainMenu::UpdateBill) => update_bill(&mut bills),
            None => break,
        }
    }
    None
}

fn main() {
    run_program();
}
