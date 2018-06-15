fn main() {
    let x = 1; //x ouns 1
    let s = String::from("String!");
    //y ouns reference to s
    let y = &s;
    println!("{}", s);
}

//fn take(v: Vec<i32>){
//    println!("We took v: {}", v[10] + v[100]);
//}

//fn cop(a: i32, b: i32){
//    println!("{}", a + b);
//}

//re takes vector and returns vector
fn re(v: Vec<i32>) -> Vec<i32> { 
    println!("{}", v[120] + v[111]);
}

fn borrow1(v: &Vec<i32>) {
    println!("{}", (*v)[10] + (*v)[12]);
}

fn borrow2(v: &Vec<i32>){
    println!("{}", v[10] + v[11]);
}

fn main() {
    let v = Vec::new();
    
    //let a = 32;
    //let b = 45;
    //cop (a,b);
    //println!("We have a: {} and b: {}", a, b);
    //take(v);
    let mut v = Vec::new();

    for i in 1..1000{
        v.push(i);
    }

    v = re(v);
    println!("Still own v: {} {}", v[0], v[1]);
    borrow1(&v);
    println!("Still own v: {} {}", v[0], v[1]);
    borrow2(&v);
    println!("Still own v: {} {}", v[0], v[1]);
    //println!("{}", v[0]);
    println!("Finished!");
}

//refence is a object
