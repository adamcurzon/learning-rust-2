struct Color {
    name: String,
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    let black = Color {
        name: "black".to_string(),
        red: 0,
        green: 0,
        blue: 0,
    };

    println!("The color is: {}", black.name);
    println!(
        "Black = rgb({}, {}, {})",
        black.red, black.green, black.blue
    );
}
