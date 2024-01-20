#![allow(unused)]

use prelude::*;
use type_state_pattern::web::RequestBuilder;

mod error;
mod prelude;

fn main() -> Result<()> {
    let req_builder = RequestBuilder::new()
        .url("https://some-url.com/task/123")
        .method("GET");

    let req_builder = req_builder.header("token", "user_uuid.exp.sign");

    let req = req_builder.clone().build();
    println!("{req:#?}");

    let req = req_builder.header("Client-Version", "1.2").build();

    println!("{req:#?}");
    Ok(())
}
