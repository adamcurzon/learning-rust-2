struct Teacher {
    first_name: String,
    last_name: String,
}

trait Teach {
    fn teach(&self) -> String;
}

impl Teach for Teacher {
    fn teach(&self) -> String {
        format!("{} {} is teaching!", self.first_name, self.last_name)
    }
}

fn main() {
    let adam: Teacher = Teacher {
        first_name: "Adam".to_string(),
        last_name: "Curzon".to_string(),
    };

    println!("Teacher 01: {}", adam.teach());
}
