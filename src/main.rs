// two process first build and then run

// cargo init - initialize a rust project
// cargo run - build and run rust project
// cargo build - building a rust project
// cargo build --release - to build a optimized release version

// variables
// we can define variables with let
// we can assign a type to variable or it can infer it as well

// fn main() {
//     // let x= 40;
//     let x:i32 = 40;

//     print!("{}", x);
// }

// i8 -- 8 bit signed integer
// i16 -- 16 bit signed integer
// i32 -- 32 bit signed integer
// i64 -- 64 bit signed integer
// i128 -- 128 bit signed integer


// u8 -- 8 bit unsigned integer
// u16 -- 16 bit unsigned integer
// u32 -- 32 bit unsigned integer
// u64 -- 64 bit unsigned integer
// u128 -- 128 bit unsigned integer

// f16 -- 16 bit decimal
// f32 -- 32 bit decimal
// f64 -- 64 bit decimal
// f128 -- 128 bit decimal

use std::{collections::HashMap, fs};

fn main() {
    let x:i8 = -40;
    let y:u8 = 30;
    let z:f32 = 20.202020; 

    // print!("x: {}", x);
    // print!("y: {}", y);
    // print!("z: {}", z);
    println!("x: {}, y: {}, z: {}", x, y, z);

    // this wont let us compile the code
    // literal out of range error
    // let a: i8 = 1000;

    // this is a runtime error
    // compiler wont catch this error
    // compiler never runs your code, it will do static analysis
    // let mut a: i8 = 10;
    // for _i in 0..1000 {
    //     a = a+100;
    // }

    // Booleans
    let is_true: bool = true;

    if is_true {
        println!("is_true");
    }

    // strings
    let greeting: String = String::from("Good afternoon");
    // let greeting: &str = "Good afternoon";
    println!("{}", greeting);

    // conditional & loops

    if is_true {
        println!("is true");
    }else if !is_true {
        println!("is false");
    }else {
        println!("Nothing");
    }

    // 0 to n-1
    for i in 0..10 {
        print!("{}", i);
    }
    print!("\n");

    let sentence: String = String::from("bla1 bla2 bla3");
    let first_word: String = get_first_word(sentence);
    println!("{}", first_word);

    let a:i32 = 32;
    let b:i32 = 32;
    let sum = do_sum_with_return(a, b);
    println!("{}", sum);
    do_sum_without_return(a, b);
    
    // println!("Hello world");

    // Memory managment
    
    // Garbage collector
    // - No dangling pointer or memory issues.
    // - Example - Java, JS

    // Whenever we run a program it allocates & deallocates memory on the RAM.
    // JS code
    // function main() {
    //     runLoop();
    // }
    // function runLoop() {
    //     let x = [];
    //     for(let i=0; i<10000; i++){
    //         x.push(1);
    //     }
    //     console.log(x);
    // }
    // main();

    // as the runLoop function runs, a new array is pushed to RAM, as soon as the control reaches after the runLoop function the array is no longer needed
    // x needs to exist inside the scope of runLoop function so it is garbage collected / x can be deallocated from the RAM.
    // and eventually garbage collected


    // Manual
    // You allocate & deallocate memory yourself
    // Can lead to dangling pointers/memory issues.
    // Example - C
    // Mallock & Callock in c

    // The RUST way
    // Manual memory managment but with strict rules so no dangling points/memory issues.
    // If you dont fulfil these rules code will not compile
    // Not having a garbage collector is the key reason why rust is fast

    // Rust achives this by using
    // - Mutability
    // - Heap & Stack
    // - Ownership model
    // - Borrowing & references
    // - Lifetimes

    // Mutability
    // Immuatable variables represnt variables whose value cant be changed once assigned.
    // By default all variables in rust are immutable beacuse
    // Immutable data is inherently thread-safe because if no thread alters the data then no sycnhronization is needed when data is accessed concurrently.
    // Compiler already knows that certain data will not change so it optimizes code better.

    // Error -- cannot mutate immutable variable `unique`rust-analyzerE0384
    // let unique = 34;
    // unique = 2 

    // let mut unique = 34;
    // unique = 36;

    // Stack vs heap
    // Stack: fast allocation & deallocation. Rust uses stack for primitive data types & data where size is known at compile time.
    // Heap: Used to data that can grow at runtime, eg. vector, strings.

    // Stack - Numbers, booleans, fixed size arrays
    // Heap - Strings, Vectors
    
    // Ownership
    // Each piece of data in rust has a single owner at any given time. whenever the owner goes out of the scope rust
    // automatically deallocates the memory associated with the data preventing memory leaks

    // every variable created in heap has its owner in stack if the stack frame gets pop the heap variable is removed.
    // if we ever try to change the owner the previos one dies.

    // let s1 = String::from("hello");
    // let s2:String = s1;

    // println!("{}", s2);
    // println!("{}", s1);

    // error - borrow of moved value: `s1` value borrowed here after move

    // s1 is no longer the owner of "hello" it ownership is passed on to s2
    // it is done to ensure that no two variables can point to same value if it is not done we will have the issue of dangling pointer

    // Borrowing & References
    // Most of the time, we'd like to access data without taking ownership over it.
    // To accomplish this, Rust uses a borrowing mechanism. Instead of passing objects by value (T), objects can be passed by reference (&T).
    // The compiler statically guarantees (via its borrow checker) that references always point to valid objects.
    // That is, while references to an object exist, the object cannot be destroyed.

    // let s1 = String::from("hello");
    // let s2: &String = &s1;

    // println!("{}", s2);
    // println!("{}", s1);

    // If the borrower dies the data is not cleared, data is cleared only when the owner dies.

    // let mut s1 = String::from("hello");
    // println!("{}", s1);
    // update_str(&mut s1);
    // println!("{}", s1);

    // Important
    // A variable can only have one mutable reference 
    // it cannot have multiple mutable or immutable references if one mutable reference is already there.

    // multiple immutable references are allowed but if there is one mutable reference no more references are allowed.

    // why??
    // To avoid data races/inconsistent behaviour
    // If someone makes an immutable reference they dont expect the value to change suddenly.
    // If more than one mutable references happen then there is a possibility of a data race and synchronization issues

    // let mut s1 = String::from("hello");
    // let s2: &mut String = &mut s1;
    // println!("{}", s1);
    // println!("{}", s2);

    // Structs
    // struts let you structure the data together.
    // unit struct & tuple struct
    struct User{
        active: bool,
        username: String,
        email: String,
    }
    let user1 = User {
        active: true,
        username: String::from("Kartik2__1"),
        email: String::from("kartikg02013@gmail.com")
    };

    println!("struct user -- {} {} {}", user1.username, user1.active, user1.email);

    // Implementing structs
    // One can attach functions to instances of structs

    struct Rect {
        height: u32,
        width: u32,
    }

    // self is similar to this keyword of Javascript
    // self is the current struct
    impl Rect {
        fn area(&self) -> u32 {
            // self.width*self.height
            return self.width*self.height;
        }
        fn debug(){
            println!("Debug funtion")
        }
    }

    let rect = Rect {
        width:30,
        height: 30,
    };
    println!("Area {}", rect.area());
    // debug function does not have self as argument so we can't call it on the rect variable but we can call it on struct itself
    // similar to static function of JS
    Rect::debug();

    // Enums
    // Enums in rust are similar to enums in typescript. They allow us to define a type by enumerating its possible variants.
    // Makes your code more strict
    
    // enum Direction {
    //     North,
    //     East,
    //     West,
    //     South,
    // }

    // let my_direction = Direction::North;

    // Pattern Matching

    // enum with values
    // enum Shape {
    //     Circle(f64),
    //     Square(f64),
    //     Rectangle(f64, f64),
    // }

    let circle: Shape = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(4.0, 6.0);

    println!("Area of circle: {}", calculate_area(circle));
    println!("Area of square: {}", calculate_area(square));
    println!("Area of rectangle: {}", calculate_area(rectangle));


    // Generics
    struct Point<T> {
        x:T,
        y:T,
    }

    // struct Points<A, B>{
    //     x:A,
    //     y:B,
    //     z:B,
    // }

    // let points: Points<i32, f32> = Points{x:5, y:10.0, z:10.0};


    let integer_point: Point<i32> = Point{x:5, y:10};

    let string_point: Point<String> = Point{x:String::from("5"), y:String::from("10")};

    println!("Integer point: ({}, {})", integer_point.x, integer_point.y);
    println!("String point: ({}, {})", string_point.x, string_point.y);


    // Error handeling
    // Rust does error handeling using Result Enum
    // Enum with generic type
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let res = fs::read_to_string( "example.txt");

    match res {
        Ok(content) => {
            println!("file content: {}", content);
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    println!("chalalala");

    // Option enum
    // The option enum in Rust was introduced to handle the concept of nullability in a safe and expressive way.
    // Rust does not have null.

    // pub enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let my_str = String::from("Kartik");

    match find_first_a(my_str){
        Some(index) => println!("The letter 'a' is found at index: {}", index),
        None => println!("The letter 'a' is not found in the string."),
    }

    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    println!("{}", mutable);

    // Error! The type of a variable can't be changed.
    // mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;
    println!("{}", mutable);


    // collections --> the data these collections point to is stored on the heap cause it is dynamic data (size can change)

    // Vectors --> similar to vectors in c++ a dynamic array where we can push and pop elements
    // it allows us to store more than one value in a single data structure that puts all the values next to each other in memory contigous allocation of memory.

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(4);
    vec.push(6);

    println!("{:?}", vec);

    // let ans = even_filter(vec);

    let ans = even_filter(&vec);

    println!("{:?}", ans);

    // cannot print this as we have given the ownership to even_filter
    // println!("{:?}", vec);

    vec.pop();
    println!("{:?}", vec);

    vec.remove(1);
    println!("{:?}", vec);

    // using vec macro to initialize a vector
    let numbers = vec![1, 2, 3, 4];
    println!("{:?}", numbers);

    // Defining the type of vector as a generic
    // when we define a vector its type can be inferred or can be defined explicitly
    // vector of type i32
    // let numbers = Vec<i32> = vec![1, 2, 3, 4];

    // HashMaps
    // stores a key value pair in rust, similar to map in C++, Dict in python
    // Methods --> insert, get, remove, clear

    let mut users: HashMap<String, i32> = HashMap::new();
    users.insert(String::from("Kartik"), 22);
    users.insert(String::from("Ashvin"), 23);

    println!("{:?}", users);

    let user = users.get("Kartik");

    // println!("{}", user.unwrap());

    match user{
        Some(u) => println!("{}", u),
        None => println!("Not found"),
    }



}

fn find_first_a(s: String) -> Option<i32> {
    for(index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    // let area:f64 = match shape {
    //     Shape::Circle(radius) => std::f64::consts::PI*radius*radius,
    //     Shape::Square(side_length) => side_length*side_length,
    //     Shape::Rectangle(width, height) => width*height,
    // }
    // return area;

    match shape {
        Shape::Circle(radius) => std::f64::consts::PI*radius*radius,
        Shape::Square(side_length) => side_length*side_length,
        Shape::Rectangle(width, height) => width*height,
    }
}

fn update_str(s: &mut String){
    s.push_str(" world");
}

fn do_sum_with_return(a: i32, b:i32) -> i32{
    return a+b;
}

fn do_sum_without_return(a: i32, b:i32){
    println!("{}", a+b);
}

fn get_first_word(sentence: String) -> String {
    let mut ans: String = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}

fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
// fn even_filter(vec: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    
    for val in vec{
        if val % 2 == 0 {
            // ans.push(val);
            ans.push(*val);
        }
    }

    return ans;
}