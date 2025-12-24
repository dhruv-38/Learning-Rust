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

// fn main(){
//     let mut s1=String::from("dhruv");
//     let s2 =&mut s1;
//     s2.push_str(" choudhary");
//     let s3 =&mut s1; //cannot have 2 mutable reference
//     let s4=&s1; //cannot have read only reference when there is one mutable reference
//     println!("string is {}",s1);
// }

// 11 Collections

//Vector
// fn main(){
//     let mut vec= Vec::new();
//     vec.push(1);
//     vec.push(2);
    
//     let vec2= vec![1,2,3,4];// aslo way to initialise a vector using a macro

//     println!("{:?}",even_filters(vec));
// }

// fn even_filters(vec: Vec<i32>) -> Vec<i32>{
//     let mut new_vec= Vec::new();
//     for val in vec{
//         if val % 2 ==0{
//             new_vec.push(val);
//         }
//     }
//     return new_vec;
// }

//Hashmaps
// use std::collections::HashMap;
// fn main (){
//     let mut users= HashMap::new();
//     users.insert(String::from("dhruv"), 20);
//     users.insert(String::from("raman"),22);

//     let first_user= users.get("dhruv");

//     match first_user {
//         Some(age)=> println!("age is {}", age),
//         None=> println!("User not found in the db")
//     }
// }

//12

//Iterators

fn main(){
    let nums= vec![1,2,3];
    let v1_iter= nums.iter();

    for val in v1_iter{
        println!("Got:{}",val);
    }

    let mut v1=vec![1,2,3];
    let v1_iter= v1.iter_mut();

    for val in v1_iter{
        *val= *val+1;
    }

    println!("{:?}",v1);

    let mut v_iter=v1.iter_mut();

    while let Some(val)=v_iter.next(){
        println!("{}",val);
    }

    let nums1= vec![1,2,3];
    let v2_iter= nums1.into_iter();

    for val in v2_iter{
        println!("Got:{}",val);
    }

    
}