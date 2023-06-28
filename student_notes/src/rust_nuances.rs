// ownership, borrowing, lifetimes, references, borrow checker, and pointers
fn bf1(s: &String) {
    println!("{s}")
}

fn bf2(s: String) {
    println!("{s}")
}

fn borrow_func() {
    let string1: String = "Hello".to_string();
    bf1(&string1);
    println!("================");
    bf2(string1.clone());
}

fn mov_func() {
    let str1: String = "hello".to_string();
    let str2: String = str1.clone();
}

pub fn main() {
    borrow_func();
    mov_func();
}
