//A GUESSING GAME PROGRAM THAT GENERATES A RANDOM INTEGER BETWEEN 1 AMD 100.
//IT WILL THEN PROMPT THE PLAYER TO ENTER A GUESS. AFTER A GUESS IS ENTERED,
//THE PROGRAM WILL INDICATE WHETHER THE GUESS IS TOO LOW OR TOO HIGH.
// IF THE GUESS IS CORRECT, 
// // A CONGRATULATORY MESSAGE IS SHOWN ON THE SCRREEN


//Step1: guessing game program will ask for user input, 
// and check that the input is in the expected form, 
// to start, we;;ll allow the player to input a guess.

// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;


//NEED TO CHECK THIS OUT I DON'T UNDERSTAND THE CONCEPT OF 
//EXTERNAL LIBRARIES IN RUST

// I have also forgotten the difference between libraries and crates

// fn main() {
//    println!("Guess the number");
//    println!("Input your guess");
   
//    let secret_number = rand::thread.rng().gen_range(1..=100);

//    let mut guess = String::new();

//    io::stdin()
//    .read_line(&mut guess).expect("Failed to read appended String");

//    let guess:u32 = match guess.trim().parse(){
//     Ok(num) => num ,
//     Err(_)  => continue,
//    };

//    println!("Write the guess , {guess}" );

//    match guess.cmp(&secret_number){
//     Ordering::Less => println!("Too small!"),
//     Ordering::Greater => println!("Too big !"),
//     Ordering::Equal => {
//       println!("You win!");
//       break;
//     }
//   //  }


//}}

// fn main() {
// let guess: u8  = 0;

// println!("The number is {guess} " )
// }


// fn main(){
//     // let sum = 5 + 10 ;
//     // let subtraction = 10 - 5;
//     // let product = 5 * 6;

//     let remainder = 5 % 3;
//     println!("The value of {remainder}")

// }

// fn main(){
//     // let t : bool = true;
//     // println!("What Boolean Value is {t} ?")
//     let z: char = 'Z';
//     println!("The data type of z is {z}")
// }


 
// fn main() {
// let tup : (i32 , f64 , u8) = (500, 6.4 ,1);

// println!(" Print {0}" , tup.0)
// }


// I DON'T UNDERSTAND USIZE 
//Based on literal sense if that grammar is actually correct
//it should mean or return the 
//data type of the elements in the array/set

// use std::io;

// fn main(){
//     let a = [ 1 , 2, 3, 4 , 5];
//     println!("Please an enter an array index");

//     let mut index = String::new();

//     io::stdin()
//     .read_line(&mut index).expect("Appended index string");

//     let index: usize = index
//        .trim()
//        .parse()
//        .expect("Index entered was not a number");

//     let element = a[index];


//     println!("The value of the element at index {index} is {element}")
// }

// I have forgotten what runtime means

// Why is there so much talk on Rust & storage / memory

// Rust memory safety principles

// Why should I believe that Rust is actually memory safe

//snakecase


//  fn main(){
//     println!("Function is named main");

//     another_function();
//  }


//  fn another_function(){
//     println!("Function is named another function")
//  }


// fn main() {
//    let x = plus_one(5);
// }

// fn plus_one(x: i32) -> i32 {
//    x + 1
// }



// fn main() {
//  let x = plus_one(5);
//   println!("The value of x is : {x}")

// }

// fn plus_one(x: i32) -> i32{
//    x + 1
// }

// fn main(){
//    let x:i32 ;

//    if x < 5 {
//       println!("The condition is true");
//    }

//       else {
//          println!("The condition is false")
//       }
   
// }



// lore about keywords being an expression in Rust


// fn main() {
//    let condition = true;
//    let number = if condition {5} else {6};

//    println!("The value of number is {number}")
// }


// fn main() {
//    let condition = true;
//    let number = if condition {5} else {6};

//    loop{
//       println!("The value of number is {number}")
// }
// }




// fn main() {
//    let mut counter = 0;

//    let result = loop {
//       counter +=1 ;

//       println!("Keep it rolling {result}");

//       if counter == 10 {
//          break counter * 2;
//       }
//    };

// }



// fn main(){
//    let mut count = 0 ;
//    'counting_up: loop {
//       println!("count = {count}");
//       let mut remaining = 10;

//       loop{
//          println!("remaining = {remaining}");
//          if remaining == 9{
//             break;
//          }

//          if count == 2 {
//             break 'counting_up;
//          }

//          remaining -= 1 ;
//       }

//       count +=1
//    }
// }

// use std::io;
// fn main(){

//    let mut s = "hello";


   
//    io::stdin()
//    .read_line(&mut s).expect("Failed to read appended String");
//     s.push_str(", world !");
    
//     println!("{s}")
// }


// I don't understand ownership in Rust (Heap & stack) ::eyes

//Ownership Rules

//Each value in Rust has an owner
//There can only be one owner at a time
//When the owner goes out of scope, the value will be dropped


//what is a garbage 
// fn main(){
// let  s1 ="hello";
// let s1 = String::from("hello");
// let s2 = s1 ;

// println!("{s2} , world !" )
// }

// fn main(){
    
// let x = 5;
// let y = x;

// println!("x = {x} , y = {y}")
// }


// fn main(){
// let s1 = "hello";    
// let s1 = String::from("hello");
// let (s2, len) = calculate_length(s1);

// println!("The length of {s2} is {len}")
// // }


// fn calculate_length(s: String) -> (String , usize){
// let length = s.len();

// (s, length)
// }



// Convert farenheit to degree Celsius 
// use std::io;
// fn main(){

//  println!("Input your value of farenheit to be converted to celsius :");

//     let mut farenheit  = String::new();

//     io::stdin()
//     .read_line(& mut farenheit).expect("Failed to read line");
   

// let farenheit :f64 = farenheit.trim().parse().expect("Successfully converted to another data type");


//     let  celsius = (farenheit - 32.0) *  5.0/9.0;

//     println!("Degree {farenheit} converted to celsius degree is {celsius} degree")
// }

// Generate the nth Fibonacci number

//Fn = Fn-1 + Fn-2

// where:
use std::io;

fn main(){
    
let a  = (1.0/5.0_f64.sqrt())*((1.0 + 5.0_f64.sqrt())/2.0);

let b = (1.0/5.0_f64.sqrt())*((1.0 - 5.0_f64.sqrt())/2.0);

    let mut n  = String::new();

    io::stdin()
    .read_line(& mut n).expect("Failed to read nth term");
   

let n :i32 = n.trim().parse().expect("Successfully generated the nth term of the Fibonacci sequence");

if n >=2 {
    
let fibonacci = a*(((1.0 + 5.0_f64.sqrt())/2.0).powi(n)) + b*(((1.0 - 5.0_f64.sqrt())/2.0)).powi(n);

println!("Here is the {n}th term of the fibonacci sequence :{fibonacci}")
}
}






// Print the lyrics to the Christmas carol "The twelve days of Christmas" , taking advantage of the repetition of the song;
