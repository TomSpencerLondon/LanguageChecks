fn main() {
    println!("Hello, world!");
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}, world!", s1);
    println!("{}, world!", s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
