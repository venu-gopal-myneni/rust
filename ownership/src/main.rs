fn main() {

    let s = String::from("hello");
    println!("{}", s);
    let p = s;
    // let p = &s;
    let k = 5;
    let r = k;

    // println!("{s} ,{p}, {k}, {r}");
    println!("{p}, {k}, {r}");
}
