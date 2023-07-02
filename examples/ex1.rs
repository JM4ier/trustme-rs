use trustme::*;

fn main() {
    let mut x: i32 = 0;
    let ptr = (&mut x) as *mut i32;
    trustme! {*ptr = 42};
    println!("{x}");
}
