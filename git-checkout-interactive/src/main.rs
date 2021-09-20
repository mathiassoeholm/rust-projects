use regex::Regex;

use std::{
    io::{stdin, stdout, Stdout, Write},
    process::Command,
};
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::{input::TermRead, raw::RawTerminal};

fn main() {
    let output = Command::new("git").arg("reflog").output().unwrap();

    let output = String::from_utf8_lossy(&output.stdout);

    let regex = Regex::new(r"moving from ([^ ]*)").unwrap();
    let branches: Vec<_> = regex
        .captures_iter(&*output)
        .take(8)
        .map(|capture| String::from(&capture[1]))
        .collect();

    if branches.len() == 0 {
        println!("The reflog is empty, you never switched branch");
        return;
    }

    println!("Select a branch:");

    let mut selection = 0;

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let print_menu = |buffer: &mut RawTerminal<Stdout>, branches: &Vec<String>, selection| {
        for (index, branch) in branches.into_iter().enumerate() {
            if index == selection {
                write!(buffer, "👉 ").unwrap();
            } else {
                write!(buffer, "   ").unwrap();
            };

            write!(buffer, "{}\r\n", branch).unwrap();
        }
        buffer.flush().unwrap();

        // Get ready to re-print the menu by moving cursor back...
        write!(buffer, "\x1B[{}A", branches.len()).unwrap();
        // ...and clearing everything after the cursor
        write!(buffer, "\x1B[0J").unwrap();
        // Buffer is not flushed yet, so menu is still printed
    };

    print_menu(&mut stdout, &branches, selection);

    let mut cancelled = false;

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') => break,
            Key::Ctrl(c) => {
                cancelled = true;
                break;
            }
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
    }

    if !cancelled {
        write!(stdout, "You chose {}\r\n", branches[selection]).unwrap();
        stdout.flush().unwrap();

        Command::new("git")
            .arg("checkout")
            .arg(&branches[selection])
            .status()
            .unwrap();
    }
}
