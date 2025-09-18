fn main() {

    println!("Hello, world!");

    // mutability
    let mut x =4;
    println!("{}", x);

    x =5;
    println!("{}", x);

    {
        // shadowing
        let x = x +10;
        println!("{}", x);

        const Y:i16 = 20;
        println!("{}", Y);
    }

    println!("{}", x);

    // constant
    const X :i32 = 10;
    println!("{}", X);
    // println!("{}", Y);

}
