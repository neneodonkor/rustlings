fn main() {
    let mut res = 42;
    let option = Some(12);

    // Clean and clear way
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
