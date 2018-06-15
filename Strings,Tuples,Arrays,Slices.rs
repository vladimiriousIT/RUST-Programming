fn main() {
    //tuple
    let t = (1, 'v', false);
    //tuple in tuple
    let f = (2, t)
    println!("{} {} {}", t.0, t.1, t.2);
    println!("{:?}", f.1);

    let xs: [i32; 5] = [4,5,6,7,8];
    println!("{}", xs[0]);
    println!("{}", xs[3]);
    println!("{} {} {}", xs[3],xs.len(), mem::size_of_val(&xs));
    //slice array
    let ys = &xs[2..4];
    println!("{} {}", ys[0] ,ys[1]);

    let s = "String".to_string();
    //function to create a string
    let ss = String::from("String!");
    //slice of our string
    let slice = &ss[0..4];
    println!("{}",slice);
    //concatinate string
    let h = String::from("Hello, ");
    let w = String::from("Vladi :) !");
    let concStr = h + &w;
    println!("{}",concStr);
    //empty tuple / empty func to empty func () -> ()
    let tup = ();
}