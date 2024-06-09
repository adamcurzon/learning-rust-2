struct CoordSet {
    x: Option<f32>,
    y: Option<f32>,
}

fn main() {
    let my_pos: CoordSet = CoordSet {
        x: Some(1.1),
        y: None,
    };
    if !my_pos.x.is_some() || !my_pos.y.is_some() {
        println!("x or y is not set");
        return;
    }

    println!("Your coords {}, {}", my_pos.x.unwrap(), my_pos.y.unwrap());
}
