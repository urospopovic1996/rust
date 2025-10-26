fn main() {
    // 1. All integer types
    let a: i8 = -2;
    let aa: u8 = 2;
    let b: i16 = -1024;
    let bb: u16 = 1024;
    let c: i32 = -2_000_000_000;
    let cc: u32 = 2_000_000_000;
    let d: i64 = - 2_000_000_000_000;
    let dd: u64 = 2_000_000_000_000;
    let e: i128 = -2_000_000_000_000_000;
    let ee: u128 = 2_000_000_000_000_000;
    let f: isize = isize::MAX;
    let ff: usize = usize::MAX;

    println!("Signed 8-bit: {}", a);
    println!("Unsigned 8-bit: {}", aa);
    println!("Signed 16-bit: {}", b);
    println!("Unsigned 16-bit: {}", bb);
    println!("Signed 32-bit: {}", c);
    println!("Unsigned 32-bit: {}", cc);
    println!("Signed 64-bit: {}", d);
    println!("Unsigned 64-bit: {}", dd);
    println!("Signed 128-bit: {}", e);
    println!("Unsigned 128-bit: {}", ee);
    println!("isize MAX: {}", f);
    println!("usize MAX: {}", ff);

    // 2. Both floating-point types
    let x: f32 = 2.1;
    let y: f64 = -2.1;

    println!("float 32: {}", x);
    println!("float 64: {}", y);

    // 3. Boolean operations (true/false, and, or, not)
    let t = true;
    let f = false;
    let and_operation = t && f;
    let or_operation = t || f;
    let t_not = !t;
    let f_not = !f;

    println!("t: {}", t);
    println!("f: {}", f);
    println!("t and f: {}", and_operation);
    println!("t or f: {}", or_operation);
    println!("t not: {}", t_not);
    println!("f not: {}", f_not);

    // 4. Various characters including emoji
    let char1 = 'a';
    let char2 = 'B';
    let emoji = '😂';

    println!("First character: {}", char1);
    println!("Second character: {}", char2);
    println!("Third character: {}", emoji);
}
