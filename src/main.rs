#![allow(unused)]

use prelude::*;
use type_state_pattern::web::RequestBuilder;

mod error;
mod prelude;

fn main() -> Result<()> {
    let req_builder = RequestBuilder::new()
        .url("https://some-url.com/task/123")
        .method("GET");

    let req = req_builder.header("token", "user_uuid.exp.sign").build();

    println!("{req:#?}");

    Ok(())
}
