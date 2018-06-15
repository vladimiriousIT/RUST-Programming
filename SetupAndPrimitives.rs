fn main() {
    println!("Hello, Vladi! I like Rust :) ...");

    //i8, u8, i16, u16, i32, u32, i64, u64, isize, usize The number of bits that tha number takes in the memory
    //f32, f64 Float numbers
    //let mut x: u32 = 5;
    let a = 1 + 20;
    let s = 30 - 20;
    let m = 5 * 10;
    let d = 4 / 6;
    let r = 49 % 2;

    //bool: true/false
    let c: char = 'z';
    //tuples [array]
    let t: (i32, f64, char) = (42, 7.15, 'v');
    //pattern matching 
    let (z, y, x) = t; // let(_, _, x) = t;
    //arrays
    let array = [1,2,3,4,5,6,7]
    //let a1 = array[0];
}
