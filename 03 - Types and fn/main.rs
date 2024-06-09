const MY_LUCKY_NUMBER: i8 = 7;

fn main() {
    // Fails not mutatable
    // let x: [i32; 3] = [1,2,3];
    // x[0] = 2;
    // println!("{:?}", x);

    let mut y: [i32; 3] = [1, 2, 3];
    y[0] = 8;
    println!("{:?}", y);

    // i8, i16, i32, i64, i128
    let num: i64 = 1_000_000_000;
    println!("{}", num);

    let float: f64 = -1_000_000_000.0;
    println!("{}", float);

    println!("{}", MY_LUCKY_NUMBER);

    println!("{}", add_one("10"));
}

fn minus_one(a: i32) -> i32 {
    a - 1
}

fn add_one(a: &str) -> String {
    return [a, "one"].join("_");
}
