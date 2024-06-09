#[derive(PartialEq, Eq, PartialOrd)]
enum Status {
    Ok = 200,
    BadRequest = 400,
    Forbidden = 401,
    NotFound = 404,
}

struct Response {
    message: String,
    status: Status,
}

fn main() {
    let r: Response = Response {
        message: "Hello World!".to_string(),
        status: Status::Ok,
    };

    println!("{}", r.message);

    if r.status >= Status::BadRequest {
        println!("There was an error");
    }
}
