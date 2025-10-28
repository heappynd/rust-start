fn main() {
    let str = "Hello, world!";
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = String::from("initial contents");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    s1.push('c');
    println!("{}", s1);
    println!("{}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // fn add(self, s: &str) -> String
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;
    // println!("{}", s);
    let s = format!("{s1}--{s2}--{s3}");
    println!("{}", s);

    let s1 = String::from("Здравствуйте"); // 24 bytes
    // let h = s1[0]; // error: cannot index into a `String`
    let s = &s1[0..4]; // slice of first character
    println!("{}", s);
    for c in "我".chars() {
        println!("{}", c);
    }
    for b in "我".bytes() {
        println!("{}", b);
    }
}
