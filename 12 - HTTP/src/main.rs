#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    let mut users_string: String = "".to_string();
    for user in users {
        users_string = [user, users_string].join(" ");
    }
    return &users;
}

#[get("/new")]
fn new() -> &'static str {
    users.push("Adam".to_string());
    return "Added";
}

#[launch]
fn rocket() -> _ {
    pub users: Vec<String> = Vec::new();
    rocket::build().mount("/", routes![index])
}
