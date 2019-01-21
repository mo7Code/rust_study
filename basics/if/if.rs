fn main() {
    let mut x = 5;
    x = x + 1;
    let y = if x == 5 {
        10
    } else {
        9
    };
    println!("{}",y);
}
