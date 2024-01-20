#![allow(unused)]

use prelude::*;
use type_state_pattern::web::RequestBuilder;

mod error;
mod prelude;

fn main() -> Result<()> {
    let mut req_builder = RequestBuilder::new();
    req_builder
        .url("https://some-url.com/task/123")
        .method("GET");

    let req = req_builder.header("token", "user_uuid.exp.sign").build();

    println!("{req:#?}");
    Ok(())
}
