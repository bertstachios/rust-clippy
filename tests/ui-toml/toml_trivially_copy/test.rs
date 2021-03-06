#![feature(tool_lints)]
#![allow(clippy::many_single_char_names)]

#[derive(Copy, Clone)]
struct Foo(u8);

#[derive(Copy, Clone)]
struct Bar(u32);

fn good(a: &mut u32, b: u32, c: &Bar, d: &u32) {
}

fn bad(x: &u16, y: &Foo) {
}

fn main() {
    let (mut a, b, c, d, x, y) = (0, 0, Bar(0), 0, 0, Foo(0));
    good(&mut a, b, &c, &d);
    bad(&x, &y);
}
