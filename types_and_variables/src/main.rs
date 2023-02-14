#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

fn main() {
    let a: u8 = 123; // u = unsigned, 8 bits, 0 - 255
    println!("a = {} unsigned", a);

    let mut b: i8 = 0; // i = signed, 8 bits, -128 - 127
    println!("b = {} before", b);

    b = 42;
    println!("b = {} after", b);

    let c = 256;
    println!("c = {} takes up {} bytes", c, mem::size_of_val(&c));

    let z: isize = 123; // isize/usize = pointer size
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {} takes up {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d: char = 'x'; // char, 8 bytes
    println!("d = {} takes up {} bytes", d, mem::size_of_val(&d));

    let e: f32 = 2.5; // f32, 4 bytes / f64, 8 bytes
    println!("e = {} takes up {} bytes", e, mem::size_of_val(&e));

    let f: bool = true; // bool, 1 byte
    println!("f = {} takes up {} bytes", f, mem::size_of_val(&f));
}
