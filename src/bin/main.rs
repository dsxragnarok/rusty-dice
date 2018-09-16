fn main() {
    let s = "3d6";

    let idx = match s.find('+') {
        Some(i) => i,
        None => match s.find('-') {
            Some(i) => i,
            None => 0
        }
    };

    println!("idx = {}", idx);
}
