fn main() {
    let n = 2;
    if n < 5 {
        println!("true");
    }
    else{
        println!("false");
    }
    //==========
    let n1 = 6;
    if n1 % 4 == 0 {
        println!("n is divisible by 4");
    } else if n % 3 == 0 {
        println!("n is divisible by 3");
    } else if n % 2 == 0 {
        println!("n is divisible by 2");
    } else {
        println!("n is not divisible by 4, 3, or 2");
    }    
    //==========
    let c = true;

    let m = if c{
        50
    } else {
        76
    };
    println!("m: {}",m);
    //===========
    let mut c = 0;

    loop{
        println!("finite");
        c += 1;

        if c >= 10 {
            break;
        }
    }
    //=============
/*    'a: loop {
        println!("loop a");
        'b: loop {
            println!("loop b");
            'c: loop {
                println!("loop c");

                break 'b
            }
        }
        continue 'a
    }
 */   
    //=============
    let x = loop {
        break 10;
    };
    println!("x: {}", x);
    //============
    let mut z = 10;

    while z != 0 {
        println!("{}!", z);

        z = z - 1;
    }
    //===========
    let v = vec![10, 20, 30, 40, 50];
    for i in v {
        println!("i: {}", i);
    }
    //===========
    for i in 1..101 {
        println!("i: {}", i);
    }
    //===========
    let xx = 5;

    match xx{
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }
    //===========
    let xxx = 25;
    
    match xxx {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13... 19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
    //=========
    let pair = (0, -2);
    match pair {
        (0,y) => println!("y: {}", y),
        (x, 0) => println!("x: {}", x),
        _      => println!("No match"),
    }
    //=========
    let pairs = (0, -2);
    match pairs {
        (x,y) if x == y => println!("Equal"),
        (x,y) if x + y == 0 => println!("Equal zero"),
        (x,_) if x % 2 == 0 => println!("X is even"),
        _      => println!("No match"),
    }
    //=========
    let pp = 5;
    match pp {
        n @ 1 ... 12 => println!("n: {}", n),
        n @ 13 ... 19 => println!("n: {}", n),
        _ => println!("no match")
    }
    //=========
//    let ppp = 5;
//    
//    let nm = match pp {
//        nm @ 1 ... 12 => nm,
//        nm @ 13 ... 19 => nm,
//        _ => 0,
//    };
//    println!(!"nm: {}", nm);
//    }
}