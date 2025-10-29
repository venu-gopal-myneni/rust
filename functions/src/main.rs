fn main() {
    println!("this is main function");
    another();
    printnum(45);
    let k =printmixed(56,'v');
    // println!("{}",k);
    let p = plus_one(32);
    println!("{}",p);
}

fn another() {
    println!("this is another function");
}

fn printnum(x:i32) {
    println!("the value of x is {x}");
}

fn printmixed(i: i32, u: char) {
    println!("the value of i,u is {i} ,{u}");
}

fn plus_one(x:i32) -> i32 {
    x + 1
}