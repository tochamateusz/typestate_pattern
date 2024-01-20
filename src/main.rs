#![allow(unused)]

use prelude::*;
use type_state_pattern::task::Task;

mod error;
mod prelude;

fn main() -> Result<()> {
    println!("Hello, world!");
    let task = Task {
        title: "Title".to_string(),
        done: false,
        desc: None,
    };
    println!("{task:#?}");
    Ok(())
}
