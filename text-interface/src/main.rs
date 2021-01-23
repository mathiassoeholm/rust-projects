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
                println!("(l) List employees");
                println!("(a) Add/edit employee");
                println!("(q) Quit");

                let command = read_input();

                if command == "a" {
                    change_state(&mut state, State::NewEmployee)
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

                let department_list = employee_db.entry(department).or_insert(vec![]);
                department_list.push(name);

                change_state(&mut state, State::Menu)
            }
            State::ListByDepartment => {
                println!("Department:");
                let department = read_input();
            }
            State::ListAll => {
                println!("Department: ")
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
