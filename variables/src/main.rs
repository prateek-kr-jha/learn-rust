fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    let tup: (i32, f64, u8) = (-250, 6.4, 1);
    // let (x, y, z) = tup;
    println!("{} {} {}", tup.0, tup.1, tup.2);
    println!("The value of x is: {x}");
}
