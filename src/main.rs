fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
  // Notice that there will not be a ; at the end of the line
  // if you do that, it will error out in mismatched types
    x + 1
}