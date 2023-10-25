#[macro_use]
extern crate rocket;
use rocket::response::content;

use ibex::prelude::*;

#[get("/")]
fn index() -> content::RawHtml<String> {
    content::RawHtml(
        view! {
            HEAD { title { "Hi!" } }
            h1 { "Hello!" }
        }
        .render(),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
