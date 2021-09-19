use regex::Regex;
use std::process::Command;

fn print_menu(branches: &Vec<String>, selection: usize) {
    for (index, branch) in branches.into_iter().enumerate() {
        if index == selection {
            print!("👉 ");
        };

        print!("{}\n", branch);
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

    let branches_len = branches.len();
    let clear_menu = || {
        print!("\x1B[{}A", branches_len);
    };

    let mut selection = 0;

    print_menu(&branches, selection);
    clear_menu();
    print_menu(&branches, selection);
    clear_menu();
    print_menu(&branches, selection);
}
