use std::ops::Add;
// rust 中的类 class


// struct 定义DATA impl定义方法
// trait 抽象行为 动作
// 其他语言class 中同时定义数据和属性
struct Point<T>{
    x: T,
    y: T,
}

// impl 提前声明
// impl<T> Point<T> {
//     fn new(x: T, y: U) -> Point<T> {
//         Point {
//             x,
//             y
//         }
//     }

//     pub fn get_x(&self) ->&T {
//         &self.x
//     }
// }

// 如果不使用new 

impl<T> Point<T> {
    // &self 表示该方法对 Rectangle 的不可变借用
    pub fn get_x(&self) ->&T {
        return &self.x
    }

    pub fn get_y(&self) -> &T {
        return &self.y
    }
    // &mut self 表示可变借用
    pub fn set_x(&mut self, x: T) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: T) {
        self.y = y;
    }

    pub fn get_distance(self) -> T 
    where T: Add<T, Output=T>
    {
        self.x + self.y
    }

    // self 表示 Point 的所有权转移到该方法中，这种形式用的较少
    // 调用即销毁
    pub fn void_method(self) {
        println!("void_method");
    }
}


pub trait Person {
    fn new (name: String, grade: String) -> Self;
    fn walk(&self);
    fn talk(&self);
    fn get_name(&self)-> String;
    // 兜底函数 可以被重载
    fn sing(&self) {
        println!("sing");
    }
}

pub struct Student {
    name: String,
    grade: String
}

impl Person for Student {
    fn new (name: String, grade: String) -> Student {
        Student {
            name,
            grade
        }
    }

    fn walk(&self){
        println!("walk");
    }

    fn talk(&self){
        println!("talk");
    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }
}


fn main() {
    let mut point = Point { x: 1.0, y: 2.0 };

    let xiaoming = Student::new(String::from("小明"), "456".to_string());
    let x =  point.get_x();
    println!("{}, {}", "point_x is", x);
    point.set_x(3.0);
    println!("{}, {}", "point_x is",  point.get_x());
    let dist = point.get_distance();
    println!("{}, {}", "point_dist is",  dist);
    let name = xiaoming.get_name();
    println!("{}, {}", "student name is",  name);
}

