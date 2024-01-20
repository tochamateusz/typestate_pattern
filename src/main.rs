#![allow(unused)]

use prelude::*;
use type_state_pattern::task::Task;

mod error;
mod prelude;

fn main() -> Result<()> {
    println!("Hello, world!");
    let task = Task::default();
    println!("{task:#?}");
    Ok(())
}
