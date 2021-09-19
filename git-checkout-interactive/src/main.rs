use regex::Regex;

use std::{
    io::{stdin, stdout, Stdout, Write},
    process::Command,
};
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::{input::TermRead, raw::RawTerminal};

fn print_menu(stdout: &mut RawTerminal<Stdout>, branches: &Vec<String>, selection: usize) {
    for (index, branch) in branches.into_iter().enumerate() {
        if index == selection {
            write!(stdout, "👉 ").unwrap();
        };

        write!(stdout, "{}\r\n", branch).unwrap();
    }
}

fn main() {
    // Get recently checked out branches
    // Show interactive menu with list of branches
    // Checkout selected branch

    let output = Command::new("git").arg("reflog").output().unwrap();

    let output = String::from_utf8_lossy(&output.stdout);

    let regex = Regex::new(r"moving from ([^ ]*)").unwrap();
    let branches: Vec<_> = regex
        .captures_iter(&*output)
        .take(5)
        .map(|capture| String::from(&capture[1]))
        .collect();

    // let clear_menu = || {
    //     print!("\x1B[{}A", branches_len);
    // };

    let mut selection = 0;

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // print_menu(&branches, selection);
    // print_menu(&branches, selection);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') => print!("Enter"),
            Key::Ctrl(c) => break,
            Key::Up => {
                selection = if selection == 0 {
                    branches.len() - 1
                } else {
                    selection - 1
                };
            }
            Key::Down => {
                selection += 1;
                selection %= branches.len();
            }
            _ => {}
        }

        print_menu(&mut stdout, &branches, selection);

        stdout.flush().unwrap();

        write!(stdout, "\x1B[{}A", branches.len()).unwrap();

        // cursor::Up(branches.len() as u16);
    }
}
