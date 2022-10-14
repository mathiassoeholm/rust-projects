use itertools::Itertools;
use regex::Regex;

use std::{
    cmp,
    io::{stdin, stdout, Stdout, Write},
    process::Command,
};
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::{input::TermRead, raw::RawTerminal};

fn main() {
    let output = Command::new("git").arg("reflog").output().unwrap();

    let output = String::from_utf8_lossy(&output.stdout);

    let regex = Regex::new(r"moving from ([^ ]*) to (.*)").unwrap();

    let res = regex.captures(&*output);
    if let None = res {
        println!("The reflog is empty, you never switched branch");
        return;
    };

    let current_branch = res.unwrap().get(2).unwrap().as_str();

    let branches: Vec<_> = regex
        .captures_iter(&*output)
        .map(|capture| String::from(&capture[1]))
        .filter(|branch| branch != current_branch)
        .unique()
        .collect();

    println!("Select a branch:");

    let menu_size = cmp::min(5, branches.len());
    let mut selection = 0;
    let mut scroll = 0;

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let print_menu =
        |buffer: &mut RawTerminal<Stdout>, branches: &Vec<String>, selection, scroll| {
            for (index, branch) in branches.into_iter().enumerate().skip(scroll).take(5) {
                if index == selection {
                    write!(buffer, "👉 ").unwrap();
                } else {
                    write!(buffer, "   ").unwrap();
                };

                write!(buffer, "{}\r\n", branch).unwrap();
            }
            buffer.flush().unwrap();

            // Get ready to re-print the menu by moving cursor back...
            write!(buffer, "\x1B[{}A", menu_size).unwrap();
            // ...and clearing everything after the cursor
            write!(buffer, "\x1B[0J").unwrap();
            // Buffer is not flushed yet, so menu is still printed
        };

    print_menu(&mut stdout, &branches, selection, scroll);

    let mut cancelled = false;

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') => break,
            Key::Ctrl(c) => {
                cancelled = true;
                break;
            }
            Key::Up | Key::Char('k') if selection > 0 => {
                selection -= 1;

                if selection < scroll {
                    scroll -= 1;
                }
            }
            Key::Down | Key::Char('j') if selection < branches.len() - 1 => {
                selection += 1;
                selection %= branches.len();

                if selection >= scroll + menu_size {
                    scroll += 1;
                }
            }
            _ => {}
        }

        print_menu(&mut stdout, &branches, selection, scroll);
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
