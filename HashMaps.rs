use std::collections::HashMap;
fn main() {
    let mut hm = HashMap::new();

    hm.insert(String::from("random"), 12); //inserting string, number
    hm.insert(String::from("strings"), 49);
    hm.remove(&String::from("strings"));

    for (k, v) in &hm{
        println!("{}: {}", k, v);
    }

    match hm.get(&String::from("random")) {
        Some(&n) => println!("{}", n),
        _ => println!("no match"),
    }

    match hm.get(&String::from("no key")) {
    Some(&n) => println!("{}", n),
    _ => println!("no match"),
    }
}
///////////////////////////
use std::collections::HashMap;
fn main() {
    let s = Some('c');

    match s{
        Some(i) => println!("{}", i),
        _ => {}
    }

    if let Some(i) = s {
        println!("{}", i);
    } else {
        {}
    }
}
//////////////////////////
fn main() {
    let mut s = Some(0);

    loop {
        match s {
            Some(i) => if i > 19{
                println!("Quit");
                s = None;
            } else {
                println!("{}", i);
                s = Some(i + 2);
            },
            _ => {
                break;
            }
        }
    }
}
////////////////////////
fn main() {
    let mut s = Some(0);

    loop {
        match s {
            Some(i) => if i > 25{
                println!("Quit");
                s = None;
            } else {
                println!("{}", i);
                s = Some(i + 2);
            },
            _ => {
                break;
            }
        }
    }
//up == down <- == ->
    while let Some(i) = s {
        if i > 19 {
            println!("Quit");
            s = None;
        } else {
            println!("{}", i);
            s = Some(i + 2);
        }
    }
}