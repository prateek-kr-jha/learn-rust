fn main() {
    println!("Hello, world!");

    another_function(5, 'X');
}

fn another_function(x: i32, unit_label: char) {
    println!("the value of x is {x} {unit_label}");
}
