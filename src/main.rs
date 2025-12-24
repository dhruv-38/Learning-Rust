// 1

// fn is_even(n:i32)->bool{
//     if n%2 == 0 {
//         return true;
//     }
//     else{
//         return false;
//     }
// }
// fn main(){
//     println!("{}",is_even(31))
// }

// 2

// fn main(){
//     let _x = 4;
//     println!("{}",fib(3));
// }

// fn fib(num:i32) -> i32 {
//     let mut first =0;
//     let mut second =1;
//     if num==0{
//         return first;
//     }
//     if num==1{
//         return second;
//     }

//     for _ in 0..num-1{
//         let temp = second;
//         second= second+first;
//         first= temp;
//     }
//     return second;
// }


//3

// fn main(){
//     let my_string= String::from("Hello world!");
//     let length =get_string_length(&my_string);
//     println!("The number of characters in the stirng is {}",length);
// }

// fn get_string_length(s:&str)->usize{
//     s.chars().count()
// }

//4

// struct User{
//     first_name:String,
//     last_name:String,
//     age: u16,
// }

// fn main(){
//     let user =User{
//         first_name: String::from("Dhruv"),
//         last_name:String::from("Choudhary"),
//         age: 20,
//     };
//     println!("{}",user.age);
// }

//5

// struct Rect {
//     width: i32,
//     height: i32,
// }

// impl Rect{
//     fn area(&self)-> i32{
//         self.width * self.height
//     }

//     fn perimeter(&self, num: i32) -> i32{
//         2 * (self.width * self.height)
//     }

//     fn debug() -> i32 {
//         return 1;
//     }
// }

// fn main(){
//     let rect1 = Rect{
//         width : 10,
//         height: 20,
//     };
//     println!("area is {}",rect1.area());
//     println!("perimeter is {}",rect1.perimeter(1));
//     println!("debug is {}",Rect::debug())
// }

//6.1

// enum Shape{
//     Rectangle,
//     Circle,
// }

// fn main(){
//     let my_shape= Shape::Rectangle;
//     print_area(my_shape);
// }

// fn print_area(shape:Shape){
//     println!("hi there");
// }

//6.2

// enum Shape{
//     Rectangle(f64,f64),
//     Circle(f64),
// }

// fn main(){
//     let rect = Shape::Rectangle(1.0, 2.0);
//     calculate_area(rect);
//     let cicle = Shape::Circle(1.0);
//     calculate_area(cicle);
// }

// fn calculate_area(shape:Shape)-> f64{
//     let area = match shape{
//         Shape::Rectangle(a,b )=> a*b,
//         Shape::Circle(r)=> 3.14*r*r,
//     };
//     return area;
// }


//7 Option enum

// fn main() {
//     let index = find_first_a(String::from("preet"));

//     match index {
//         Some(value) => println!("index is {}", value),
//         None => println!("a not found"),
//     }
// }

// fn find_first_a(s: String) -> Option<i32> {
//     for (index, char) in s.chars().enumerate() {
//         if char == 'a' {
//             return Some(index as i32);
//         }
//     }

//     return None;
// }

//8 Result enum

// use std::fs::read_to_string;

// fn main() {
//     let ans = read_from_file_dhruv(String::from("a.txt"));
// }


// fn read_from_file_dhruv(file_path: String) -> Result<String, String> {
//     let result = read_to_string(file_path); // Result
//     match result {
//         Ok(data) => Ok(data),
//         Err(err) => Err(String::from("File not read")),
//     }
// }


// 9 Package Management

// use chrono::{Local};

// fn main(){
//     let now= Local::now();
//     print!("current time is {}",now);
// }

//10 Ownership

fn main(){
    let s1=String::from("dhruv");
    let s2=s1.clone();
    println!("string is {}",s1);
}