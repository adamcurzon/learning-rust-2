const APPLE: &str = "apple";
const ORANGE: &str = "orange";
const PEAR: &str = "pear";

const FRUITS: [&str; 3] = [APPLE, ORANGE, PEAR];

pub fn show_fruits() {
    println!("{:?}", FRUITS);
}
