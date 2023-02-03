use rocket::tokio::time::{sleep, Duration};

use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Contact {
    first_name: String,
    last_name: String
}

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/about")]
fn about() -> &'static str {
    "About Us"
}

#[get("/contact-info")]
fn contact_info() -> Json<Contact> {
    let result = Contact {
        first_name: String::from("David"),
        last_name: String::from("Lebee")
    };
    Json(result)
}


#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/echo/<message>")]
fn echo(message: &str) -> String {
    message.to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, about, delay, echo, contact_info])
}
