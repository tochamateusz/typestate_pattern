#![allow(unused)]

use prelude::*;
use type_state_pattern::task::Task;

mod error;
mod prelude;

fn main() -> Result<()> {
    println!("Hello, world!");
    let task = Task {
        done: true,
        ..Task::new("cool task")
    };
    println!("{task:#?}");
    Ok(())
}
