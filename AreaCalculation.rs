use std::fmt;

#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}

//fn area(obj: &Object) -> u32 {
//    obj.width * obj.height
//    //don't need to write return it's automatic
//}

//Methods
impl Object{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show(&self) {
        println!("{}x{} with area: {}", self.width, self.height, self.area());
    }
}
//Related Functions
impl Object{
    fn new(width: u32, height: u32) -> Object {
        Object {
            width: width,
            height: height,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt:: Formatter) -> fmt::Result{
        write!(f, "({}, {}", self.width, self.height)
    }
}

fn main() {
    let o = Object {
        width: 35,
        height: 55,
    };

    let obj = Object::new(57, 83);
    o.show();
    obj.show();

    println!("{:?}", o);
    println!("{:#?}", obj); 
    //println!("{}x{} with area: {}", o.width, o.height, o.area()); //area(&o)
    //println!("{}x{} with area: {}", obj.width, obj.height, obj.area()); 
}