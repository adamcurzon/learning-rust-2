fn main() {
    let mut v: Vec<String> = Vec::new();
    v.push("Adam".to_string());
    v.push("Bob".to_string());
    v.push("Jim".to_string());

    // & means borrowed and not mutable
    for i in &v {
        println!("{} is a member", i)
    }

    // &mut means borrowed and mutable
    for i in &mut v {
        *i = [i, "*"].join("");
        println!("{} is a member", i)
    }

    // Neither means take ownership and mutable
    for i in v {
        let mut i = [i, "*".to_string()].join("");
        println!("{} is a member", i)
    }
}
