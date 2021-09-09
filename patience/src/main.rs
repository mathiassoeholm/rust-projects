#![allow(dead_code)]

use std::future::Future;

fn main() {
    println!("Hello, world!");
    let x = foo();

    // We await multiple futures and are able to handle their results individually
    // Whenever a future yields, control is given to select!, so another Future has
    // a chance to continue running

    select! {
        b <- bar().await => {
            // Do something with b
        }
        f <- foo().await => {
            // Do something with f
        }
    }
}

// How you could do cancel:
// select! {
//     done <- do_work().await => {
//         // Done
//     }
//     cancel <- are_we_cancelled().await => {
//         // cancel might just be ()??
//         // If are_we_cancelled is done before do_work, the do_work doesn't get to run more
//     }
// }

async fn foo() -> usize {
    // A future does nothing if it is not awaited
    // A series of steps that will get executed at some point in the future
    // Executing in chunks
    // yield returns all the way up to the top thing awaiting
    // yield is not actually doing a busy loop, but will be smart about when to "retry"
    // select! allows branching to multiple futures
    // You can "cancel" Futures by simply not awaiting them, because Futures won't run unless awaited
    // select! could also have been called race!
    println!("foo");
    let fut = read_to_string("file1");
    fut.await;
    6
}

async fn read_to_string(arg: &str) -> () {}

fn bar() -> impl Future<Output = ()> {
    async {}
}

// How do you run the top level future that describes the entire control flow of the application?
// Something has to run that future in a loop, that's what we call an executor.
// A primitive executor just polls futures, retrying aggresively

// tokio is an executor crate
// Provides both executor loop and lower level resources like network and timers
// So it knows when to check the future
// tokio will save file descriptors and tell the OS: wake me up if any of these changes

// You can make main async like this:
#[tokio::main]
async fn main() {}

// Actually main is not async, it just gets transformed to use tokio Runtime and `block_on(async { ....} `

// The outer executor loop is sort of similar to the event loop in JavaScript

// In general you don't have to yield yourself, but you do it through awaiting

// The standard library generally does not use async, because it depends on the executor.
// That's why tokio has its own functions for file handling for example

// When using select! you have to be careful if one branch completes while another branch is only halfway done.
// You might end up writing just half the bytes to a file, but the select! will no longer allow that remaining arm to run

// Cooperatively schedules = other things can run as long as the thing that is running yields

// spawn_blocking is a function that signals to the executor that we're about to block.
// it will then allow some futures to finish, before the blocking operation happens.

// select! expands to Rust cod that does the correct stuff. You can't say that it returns a future.

fn abc() {
    // An iterator of futures and we want to wait for all of them to complete
    let files: Vec<_> = (0..3)
        .map(|i| tokio::fs::read_to_string(format!("file{}", i)))
        .collect();

    // We don't want to do this, because it's sequential
    let file1 = files[0].await;
    let file2 = files[1].await;
    let file3 = files[2].await;

    // We can use the join operation
    // Reads all three files in parallel
    let (file1, file2, file3) = join!(files[0], files[1], files[2]);
}

// join and select means that code gets to run in parallel, but not in parallel, there's a difference.

// Our program is just a function that return 1 Future. This means that it doesn't make sense to spin up
// more than one thread. What would those other threads do? There's only one future that can be made progress on.

// We can get parrallelism by using a spawn function. It moves a future up to the executor.
tokio::spawn(hande_connection(stream))
// It's as if we gave that future (handle_connection) to the runtime instance
// The result is that there are two top-level futures, which means two thread can run them at the same time

// https://www.youtube.com/watch?v=ThjvMReOXYM 1:32:14
