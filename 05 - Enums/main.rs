#[derive(PartialEq)]
enum Day {
    Monday,
    Tuesday,
    Friday,
}

fn main() {
    let today = Day::Monday;
    if today == Day::Monday {
        println!("I like mondays!");
        return;
    }

    println!("I don't like today");
}
