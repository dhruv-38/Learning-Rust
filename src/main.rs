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

struct User{
    first_name:String,
    last_name:String,
    age: u16,
}

fn main(){
    let user =User{
        first_name: String::from("Dhruv"),
        last_name:String::from("Choudhary"),
        age: 20,
    };
    println!("{}",user.age);
}