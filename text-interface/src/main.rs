use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum State {
    Menu,
    NewEmployee,
    ListByDepartment,
    ListAll,
}

fn main() {
    let mut state = State::Menu;
    let mut employee_db = HashMap::new();

    loop {
        match state {
            State::Menu => {
                println!("Welcome to the employee database");
                println!("(l) List all employees");
                println!("(d) List employees in a department");
                println!("(a) Add employee");
                println!("(q) Quit");

                let command = read_input();

                if command == "a" {
                    change_state(&mut state, State::NewEmployee)
                } else if command == "d" {
                    change_state(&mut state, State::ListByDepartment)
                } else if command == "l" {
                    change_state(&mut state, State::ListAll)
                } else if command == "q" {
                    println!("Bye bye 👋");
                    break;
                }
            }
            State::NewEmployee => {
                println!("Employee name:");
                let name = read_input();
                println!("Department:");
                let department = read_input();

                let department_list = employee_db
                    .entry(department.to_lowercase())
                    .or_insert(vec![]);
                department_list.push(name);
                department_list.sort();

                change_state(&mut state, State::Menu)
            }
            State::ListByDepartment => {
                println!("Department:");
                let department = read_input();

                let employees = employee_db.get(&department.to_lowercase());
                match employees {
                    Some(employees) => {
                        println!("Employees in {}:", department);
                        for e in employees {
                            println!("{}", e);
                        }
                    }
                    None => println!("That department does not exist."),
                }

                change_state(&mut state, State::Menu)
            }
            State::ListAll => {
                println!("All employees:");
                for (department, employees) in employee_db.iter() {
                    println!("- {}", department);
                    for e in employees {
                        println!("  - {}", e);
                    }
                }
                change_state(&mut state, State::Menu)
            }
        }
    }
}

fn change_state(current_state: &mut State, new_state: State) {
    println!("");
    *current_state = new_state;
}

fn read_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return String::from(input.trim());
}
