use std::vec;

fn find_largest<T:Ord+Copy>(list:&[T])->T{
    let mut largest = list[0];

    for &item in list{
        if item > largest {
            largest = item;
        }
    }
    largest
}



trait Shape {
    fn area(&self)->f32;
    
}
trait  Perimeter {
    fn perimeter(&self)->f32;
}

struct Rect{
    width : f32,
    height : f32
}

struct Circle{
    radius : f32
}

struct Triangle{
    base : f32, 
    height: f32,
    side: f32
}

impl Shape for Rect {

    fn area(&self)->f32 {
        return self.width * self.height;
    }
}
impl Perimeter for Rect {
    fn perimeter(&self)->f32 {
        return  2.0 * (self.width + self.height);
    }
}

impl Shape for Circle {
    fn area(&self)->f32 {
        return 3.14 * self.radius * self.radius;
    }

    
}

impl Perimeter for Circle{
    fn perimeter(&self)->f32 {
        return  2.0 * 3.14 * self.radius;
    }
}

impl Shape for Triangle {
    fn area(&self)->f32 {
        return 0.5 * self.base * self.height;
    }
    
}

impl Perimeter for Triangle {
    fn perimeter(&self)->f32 {
        return self.base + self.height + self.side;
    }
}

fn print_area<T: Shape + Perimeter> (s: &T){
    println!("area {}", s.area());
    println!("perimeter {}", s.perimeter());
}


fn main(){

    let nums = vec![34, 34,45,54];
    let chars = vec!['a','b', 'c'];

    println!("{}", find_largest(&nums));
    println!("{}", find_largest(&chars));

    let mut stack = Stack::new();

    stack.push(34);
    stack.push(344);
    stack.push(3434);

    println!("{:?}", stack.peek());
    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());


    let r = Rect{
        width: 34.3,
        height: 34.3
    };

    let c = Circle{
        radius: 4.4,
    };

    let t = Triangle{
        base: 85.4,
        height: 34.3,
        side : 44.4
    };

    print_area(&r);
    print_area(&c);
    print_area(&t);
}





struct Stack<T>{
    elements: Vec<T>
}

impl<T> Stack<T> {

    fn new()-> Self{
        Stack{elements: Vec::new()}
        
    }


    fn push(&mut self ,item:T){
        self.elements.push(item);
    }

    fn pop(&mut self) -> Option<T>{
        return  self.elements.pop();
    }

    fn peek(&self)->Option<&T>{
        return  self.elements.last();
    }
    
}
