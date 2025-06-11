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

use std::{collections::HashMap, fmt::format, fs, iter::Sum};
// use std::thread;
use std::{sync::mpsc, thread::{self, spawn}};

fn main() {
    let x: i8 = -40;
    let y: u8 = 30;
    let z: f32 = 20.202020;

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
    } else if !is_true {
        println!("is false");
    } else {
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

    let a: i32 = 32;
    let b: i32 = 32;
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
    struct User {
        active: bool,
        username: String,
        email: String,
    }
    let user1 = User {
        active: true,
        username: String::from("Kartik2__1"),
        email: String::from("kartikg02013@gmail.com"),
    };

    println!(
        "struct user -- {} {} {}",
        user1.username, user1.active, user1.email
    );

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
            return self.width * self.height;
        }
        fn debug() {
            println!("Debug funtion")
        }
    }

    let rect = Rect {
        width: 30,
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
        x: T,
        y: T,
    }

    // struct Points<A, B>{
    //     x:A,
    //     y:B,
    //     z:B,
    // }

    // let points: Points<i32, f32> = Points{x:5, y:10.0, z:10.0};

    let integer_point: Point<i32> = Point { x: 5, y: 10 };

    let string_point: Point<String> = Point {
        x: String::from("5"),
        y: String::from("10"),
    };

    println!("Integer point: ({}, {})", integer_point.x, integer_point.y);
    println!("String point: ({}, {})", string_point.x, string_point.y);

    // Error handeling
    // Rust does error handeling using Result Enum
    // Enum with generic type
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let res = fs::read_to_string("example.txt");

    match res {
        Ok(content) => {
            println!("file content: {}", content);
        }
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

    match find_first_a(my_str) {
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
    // use std::collections::HashMap;

    let mut users: HashMap<String, i32> = HashMap::new();
    users.insert(String::from("Kartik"), 22);
    users.insert(String::from("Ashvin"), 23);

    println!("{:?}", users);

    let user = users.get("Kartik");

    // println!("{}", user.unwrap());

    match user {
        Some(u) => println!("{}", u),
        None => println!("Not found"),
    }

    users.remove("Kartik");

    println!("{:?}", users);

    users.clear();

    println!("{:?}", users);

    // given a vector of tuple convert it to hashmap

    let input_vec = vec![(String::from("Kartik"), 22), (String::from("Ashvin"), 23)];

    let mp = group_values_by_keys(input_vec);

    println!("{:?}", mp);

    // Iterators
    // The iterator pattern allows you to perform some task on a sequence of items in turn.
    // An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished.
    // When you use iterators, you don’t have to reimplement that logic yourself.
    // In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.

    let v1 = vec![1, 2, 3];

    for val in &v1 {
        print!("{} ", val);
    }

    let v1_iter = v1.iter();

    println!("{:?}", v1_iter);

    // iter method provides a way to iterate over the elements of a collection by borrowing them.
    // we can't mutate the variables since we have an immutable referece to the internal elements.
    // to mutate the variables use iter_mut method

    // Types of Iterators
    // 1. Iterating usig for loops
    // let nums = vec![1, 2, 3];
    // for value in nums {
    //     println!("{}", value)
    // }

    // 2. Iterating after creating an iterator
    // first create an iterator from vector .iter() retuns a new variable of type iterator
    // let nums = vec![1, 2, 3];
    // let iter_nums = nums.iter();
    // for value in iter_nums {
    //     // cannot mutate immutable reference
    //     // value += 1;
    //     println!("{}", value)
    // }

    // 3. Iterating using .next
    // let nums = vec![1, 2, 3];
    // let mut iter_nums = nums.iter();

    // // while there is some value next print the value
    // // next method returns a option (it will point to a number of i32 or nothing in this case)

    // while let Some(val) = iter_nums.next() {
    //     print!("{} ", val);
    // }

    // 4. IterMut
    // let mut nums = vec![1, 2, 3];
    // let iter_nums = nums.iter_mut();

    // for value in iter_nums {
    //     *value = *value + 1;
    // }

    // print!("{:?}", nums);

    // 5. IntoIter
    // it is used to convert a collection into an iterator that takes ownership of the collection
    // usefull when

    // 1. No longer need the original collection
    // 2. Need to squeeze performance benefits by transferring ownership (avoiding references)

    // let nums = vec![1, 2, 3];
    // let iter_nums = nums.into_iter();

    // for value in iter_nums {
    //     print!("{} ", value);
    // }

    // which one to use?

    // iter
    // if we want immutable reference to the inner variable and dont want to trasfer ownership

    // iterMut
    // if we want mutable references to the inner varaibles and dont want to transfer ownership

    // iterInto
    // if you want to mover the varaible into the iterator and dont want to use it afterwards

    // when we write a simple for loop it uses into_iter

    // Consuming adaptors
    // methods that call next are called consuming adaptors, because calling them uses up the iterator

    // let nums = vec![1, 2, 3];
    // let iter_nums = nums.iter();
    // let total : i32 = iter_nums.sum();

    // print!("{}", total);

    // sum will consume the iterator not the original vector so we cant use the iterator anymore.

    // Iterator adaptors
    // iterator adaptors are methods defined on the iterator trait that dont consume the iterator,
    // they produce different iterators by changing some aspect of the original iterator.

    let nums = vec![1, 2, 3];
    let iter_nums = nums.iter();

    // |x| x+1 is a closure, |x| is argument and x+1 is what we return

    // map is similar to map in js
    // let iter2 = iter_nums.map(|x| x+1);

    // for val in iter2 {
    //     print!("{} ", val);
    // }

    // filter
    // let iter3 = iter_nums.filter(|x| **x%2 == 0);

    // for val in iter3 {
    //     print!("{} ", val);
    // }

    // how to convert an iterator back to a vector?
    // simply call collect on the iterator

    // let iter2 = iter_nums.map(|x| x+1);
    // let new_nums: Vec<i32> = iter2.collect();

    // print!("{:?}", new_nums);

    // String vs Slice

    // The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.
    // When Rustaceans refer to “strings” in Rust, they might be referring to either the String or the string slice &str types, not just one of those types.
    // both types are used heavily in Rust’s standard library, and both String and string slices are UTF-8 encoded.

    // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
    // A slice is a kind of reference, so it does not have ownership.

    let mut name = String::from("Kartik");
    name.push_str(" Gupta");
    println!("{}", name);

    println!("length {}", name.len());

    // replace everything from 7 to end of the string
    name.replace_range(7..name.len(), "gupta");

    println!("{}", name);

    // slice
    // take the example of function where we have to return first word from the sentence we return a new string there
    // we have that string in original as well as in new variable as well, it will require more memory to store it.
    // and if the original string is deleted we still have the part of string stored and if the original string changes i still have old answer stored somewhere
    // so what we want is our answer should reference to the part of original string
    // this way it takes less space & changes whenever the original is changed

    // what we want is instead of returning first word from string we want a view into the original string and not to copy it over

    let word = String::from("Hello world");

    // let word2 = &word[0..5];
    let word2 = find_first_word(&word);

    // if we try to clear -> word compiler does not let us clear it
    // word.clear();

    println!("{}", word2);

    // type of s is &str we have use string literal
    let s = "Hello world";
    println!("{}", s);

    // string slices can also be applied to collections like arrays/vectors

    let arr = vec![1, 2, 3];

    let arr_slice = &arr[1..2];
    println!("{:?}", arr_slice);

    // Generics

    // let bigger_i32 = largest_i32(2, 4);
    // let bigger_char = largest_char('a', 'b');

    // println!("{}", bigger_i32);
    // println!("{}", bigger_char);

    // code repetation we can reduce this by using a generic type

    let bigger_i32 = largest(2, 4);
    let bigger_char = largest('a', 'b');

    println!("{}", bigger_i32);
    println!("{}", bigger_char);

    // Traits
    // A trait defines the functionality a particular type has and can share with other types.
    // we can use traits to define a shared behavior in an abstract way.
    // we can use trait bounds to specify that a generic type can be any type that has certian behavior.

    // traits is similar to interfaces in other languages with slight differences
    // a single struct can implement multiple traits

    let user = User2 {
        name: String::from("Kartik"),
        age: 22,
    };
    println!("{}", user.summarize());

    noftiy(&user);
    noftiy(&String::from("kartik"));

    // item: impl Summary 
    // is syntax sugar for a longer form known as trait bound
    // pub fn notify<T: Summary>(item: T) {}


    // Lifetimes
    // takes lot of time to understand why they are needed
    // lots of time compiler will help and guide in right direction
    
    // works like charm
    // let ans;
    // let str1 = String::from("small");
    // let str2 = String::from("longer");
    // ans = longest(str1, str2);

    // perfectly fine as well
    // let ans;
    // let str1 = String::from("small");
    // {
    //     let str2 = String::from("longer");
    //     ans = longest(str1, str2);
    // }

    // println!("{}", ans);

    // lets instead of passing the complete ownership of string to longest function now we pass reference

    // now the ans points to either str2 or str1 if it points to str2 str2 is not valid outside the block so how can we access it
    // classic dangling pointer behaviour
    
    // compiler will think it doesnt know the lifetime or str1 or str2 so it does not know for how many time will ans be alive

    // let ans;
    // let str1 = String::from("small");
    // {
    //     let str2 = String::from("longer");
    //     ans = longest(&str1, &str2);
    // }

    // println!("{}", ans);

    // lifetime generic annotation 'a
    let ans;
    let str1 = String::from("small");
    {
        let str2 = String::from("longer");
        // `str2` does not live long enough
        // borrowed value does not live long enough
        ans = longest(&str1, &str2);

        // return type is only valid for lifetime intersection of str1 & str2

        // the return reference will be valid as long as both the references are valid
        println!("{}", ans);
    }

    // struct with lifetimes

    struct User3<'a> {
        name: &'a str
    }

    let first_name = String::from("Kartik");
    let user = User3{name: &first_name};
    println!("The name of the user {} ", user.name);


    // Multithreading
    // machines have multiple cores(CPU's)
    // but till now we were only using single core we can do more things parallaly on different cores using multithreading

    // let handle = thread::spawn(
    //     || {
    //         for i in 1..10 {
    //             println!("hii number {i} from spawned thread");
    //         }
    //     }
    // );

    // for i in 1..50 {
    //     println!("hii number {i} from main thread");
    // };

    // wait till the handle thread is spawned
    // handle.join();

    // using move closure with threads
    // we will use move keyword with closures that are passed to thread::spawn
    // because the closure will take ownership of the values it uses from the environment
    // thus transferring ownership from one thread to another

    // compiler does not know when the thread will be executed if it is executed after the block ends
    // vec1 is removed but the thread might try to access it to prevent this memory issue we use move
    {
        // let vec1 = vec![1, 2, 3];
        // let handle = thread::spawn(|| {
        //     println!("{:?}", vec1);
        // });

        let vec1 = vec![1, 2, 3];
        let handle = thread::spawn(move|| {
            println!("{:?}", vec1);
        });
        handle.join();
    }

    // message passing
    // passing data between multiple threads
    // it lets thread communicate with each other
    // we use channels for this communication

    // use std::{sync::mpsc, thread::{self, spawn}};

    // mpsc -> multiple producer single consumer

    let (tx, rx) = mpsc::channel();

    // to create multiple producer do tx.clone()

    spawn(move|| {
        tx.send(String::from("Kartik")).unwrap();
    });

    let value = rx.recv();
    match value{
        Ok(value) => println!("{}", value),
        Err(_err) => println!("Error while recieving"),
    }




}

// fn longest(a:String, b:String) -> String {
//     if a.len() > b.len() {
//         return a;
//     }
//     else{
//         return b;
//     }
// }

// fn longest(a:&str, b:&str) -> &str {
//     if a.len() > b.len() {
//         return a;
//     }
//     else{
//         return b;
//     }
// }

// with lifetime generic annotation
fn longest<'a>(a:&'a str, b:&'a str) -> &'a str {
    if a.len() > b.len() {
        return a;
    }
    else{
        return b;
    }
}

struct User2 {
    name: String,
    age: u32,
}

// public trait summary
// normal trait where the implementor has to define it
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// default trait
pub trait Summary {
    fn summarize(&self) -> String{
        return String::from("summarize")
    }
}

// implement summary trait for user2
// we can simply say user2 struct implements summary trait
// impl Summary for User2 {
//     fn summarize(&self) -> String {
//         return format!("User {} is {} years old", self.name, self.age);
//     }
// }

// for default summary trait
impl Summary for User2 {}
impl Summary for String {}

// Traits as parameters
// this function only accept items which have implemented Summary trait
pub fn noftiy(item: &impl Summary) {
    println!("{}", item.summarize());
}

// function generic types
// <T: std::cmp::PartialOrd> tells the compiler that T can only be things which can are comparable not everything
fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn largest_i32(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    } else {
        return b;
    }
}

fn largest_char(a: char, b: char) -> char {
    if a > b {
        a
    } else {
        b
    }
}

fn find_first_word(word: &String) -> &str {
    let mut space_index = 0;
    for i in word.chars() {
        if i == ' ' {
            break;
        }
        space_index = space_index + 1;
    }

    return &word[0..space_index];
}

fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut mp = HashMap::new();
    for (key, value) in vec {
        mp.insert(key, value);
    }
    return mp;
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
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
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side_length) => side_length * side_length,
        Shape::Rectangle(width, height) => width * height,
    }
}

fn update_str(s: &mut String) {
    s.push_str(" world");
}

fn do_sum_with_return(a: i32, b: i32) -> i32 {
    return a + b;
}

fn do_sum_without_return(a: i32, b: i32) {
    println!("{}", a + b);
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

    for val in vec {
        if val % 2 == 0 {
            // ans.push(val);
            ans.push(*val);
        }
    }

    return ans;
}
