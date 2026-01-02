#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::Deref;
use std::rc::Rc;

pub fn smart_pointers() {
    // Smart pointers
    // A pointer is a general concept for a variable that contains an address in memory.
    // This address refers to, or “points at,” some other data.
    // The most common kind of pointer in Rust is a reference.
    // References are indicated by the & symbol and borrow the value they point to.
    // They don’t have any special capabilities other than referring to data, and they have no overhead.

    // Smart pointers, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities. 

    // Rust, with its concept of ownership and borrowing, has an additional difference between references and smart pointers:
    // while references only borrow data, in many cases smart pointers own the data they point to.

    // The most common smart pointers in the standard library:
    // Box<T>, for allocating values on the heap
    // Rc<T>, a reference counting type that enables multiple ownership
    // Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

    // The most straightforward smart pointer is a box, whose type is written Box<T>.
    // Boxes allow you to store data on the heap rather than the stack.
    // What remains on the stack is the pointer to the heap data.

    // Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack.
    // But they don’t have many extra capabilities either. You’ll use them most often in these situations:

    // When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    // When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    // When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

    // Using Box<T> to Store Data on the Heap
    
    let b = Box::new(5);

    println!("b: {b}");

    // We define the variable b to have the value of a Box that points to the value 5, which is allocated on the heap.
    // This program will print b = 5; in this case, we can access the data in the box similarly to how we would if this data were on the stack.
    // Just like any owned value, when a box goes out of scope, as b does at the end of main, it will be deallocated.
    // The deallocation happens both for the box (stored on the stack) and the data it points to (stored on the heap).

    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }

    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    // Computing the Size of a Non-Recursive Type
    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }
    // To determine how much space to allocate for a Message value,
    // Rust goes through each of the variants to see which variant needs the most space.
    // Rust sees that Message::Quit doesn’t need any space, Message::Move needs enough space to store two i32 values, and so forth.
    // Because only one variant will be used, the most space a Message value will need is the space it would take to store the largest of its variants.

    // Contrast this with what happens when Rust tries to determine how much space a recursive type like the List enum needs.
    // The compiler starts by looking at the Cons variant, which holds a value of type i32 and a value of type List.
    // Therefore, Cons needs an amount of space equal to the size of an i32 plus the size of a List.
    // To figure out how much memory the List type needs, the compiler looks at the variants, starting with the Cons variant.
    // The Cons variant holds a value of type i32 and a value of type List, and this process continues infinitely

    // Indirection means that instead of storing a value directly,
    // we should change the data structure to store the value indirectly by storing a pointer to the value instead.

    // Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: a pointer’s size doesn’t change based on the amount
    // of data it’s pointing to. This means we can put a Box<T> inside the Cons variant instead of another List value directly.
    // The Box<T> will point to the next List value that will be on the heap rather than inside the Cons variant.
    // Conceptually, we still have a list, created with lists holding other lists,
    // but this implementation is now more like placing the items next to one another rather than inside one another.

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // The Cons variant needs the size of an i32 plus the space to store the box’s pointer data.
    // The Nil variant stores no values, so it needs less space on the stack than the Cons variant.
    // We now know that any List value will take up the size of an i32 plus the size of a box’s pointer data.
    // By using a box, we’ve broken the infinite, recursive chain, so the compiler can figure out the size it needs to store a List value.

    // The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references.
    // When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation. 

    // Important
    // let mut a = 32;
    // let b = &mut a;

    // *b = *b + 1; // *b += 1

    // // wont be able to print a as it already has a active mutable reference so it cant have one more reference
    // println!("a: {a}, b: {b}");

    // let mut a = 32;
    // {
    //     let b = &mut a;
    //     *b = *b + 1; // *b += 1
    // }
    // now b is no more in scope so does it reference now we can again reference a
    // println!("a: {a}");

    // let mut a = 32;
    // let b = &mut a;
    // *b = *b + 1; // *b += 1

    // manually drop b
    // println!("b: {b}");
    // let _b = drop(b);

    // println!("a: {a}");

    // Treating Smart Pointers Like Regular References with Deref
    // Implementing the Deref trait allows you to customize the behavior of the dereference operator *

    // A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else
    
    // variable x holds an i32 value 5. We set y equal to a reference to x
    // let x = 5;
    // let y = &x;

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // here we set y to be an instance of a box pointing to a copied value of x
    // rather than a reference pointing to the value of x
    // let x = 5;
    // let y = Box::new(x);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // Defining Our Own Smart Pointer

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Without the Deref trait, the compiler can only dereference & references.
    // The deref method gives the compiler the ability to take a value of any
    // type that implements Deref and call the deref method to get an & reference
    // that it knows how to dereference.

    // *y == *(y.deref())

    // Implicit Deref Coercions with Functions and Methods

    // Running Code on Cleanup with the Drop Trait

    // The second trait important to the smart pointer pattern is Drop,
    // which lets you customize what happens when a value is about to go out of scope.
    // For example, when a Box<T> is dropped it will deallocate the space on the heap that the box points to.

    // we can provide an implementation for the Drop trait on any type,
    // and that code can be used to release resources like files or network connections.

    let c = CustomPointer{
        data: String::from("Kartik")
    };

    // we are not allowed to explicitly call drop method
    // error[E0040]: explicit use of destructor method
    // c.drop();

    // Rust doesn’t let us call drop explicitly because Rust would still automatically call drop
    // on the value at the end of main. This would cause a double free error because Rust
    // would be trying to clean up the same value twice.

    // We can’t disable the automatic insertion of drop when a value goes out of scope,
    // and we can’t call the drop method explicitly. So, if we need to force a value to be cleaned
    // up early, we use the std::mem::drop function.

    drop(c);

    println!("Custom pointer created");

    // Rc<T>, the Reference Counted Smart Pointer
    // In the majority of cases, ownership is clear: we know exactly which variable owns a given value.
    // However, there are cases when a single value might have multiple owners.
    // For example, in graph data structures, multiple edges might point to the same node, and that node is
    // conceptually owned by all of the edges that point to it.
    // A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it and so has no owners.

    // We have to enable multiple ownership explicitly by using the Rust type Rc<T>,
    // which is an abbreviation for reference counting. The Rc<T> type keeps track of the number of
    // references to a value to determine whether or not the value is still in use.
    // If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

    // We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program
    // to read and we can’t determine at compile time which part will finish using the data last.
    // If we knew which part would finish last, we could just make that part the data’s owner,
    // and the normal ownership rules enforced at compile time would take effect.

    // Using Rc<T> to Share Data
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // The Cons variants own the data they hold, so when we create the b list, a is moved into b
    // and b owns a. Then, when we try to use a again when creating c, we’re not allowed to because
    // a has been moved.

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));

    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // we create the list holding 5 and 10 and store it in a new Rc<List> in a.
    // Then, when we create b and c, we call the Rc::clone function and pass a
    // reference to the Rc<List> in a as an argument.

    // We could have called a.clone() rather than Rc::clone(&a), but Rust’s convention is to use
    // Rc::clone in this case. The implementation of Rc::clone doesn’t make a deep copy of all the
    // data like most types’ implementations of clone do. The call to Rc::clone only increments the
    // reference count, which doesn’t take much time. Deep copies of data can take a lot of time.

    // reference count: the implementation of the Drop trait decreases the reference count automatically when an Rc<T> value goes out of scope.

    // RefCell<T> and the Interior Mutability Pattern
    
    // Interior mutability is a design pattern in Rust that allows us to mutate data even
    // when there are immutable references to that data; normally, this action is disallowed
    // by the borrowing rules
    // To mutate the data we use unsafe code inside a data structure to bend the rules
    // unsafe code tells the compiler that we are checking the rules manually instead of relying on compiler

    // Enforcing Borrowing Rules at Runtime with RefCell<T>

    // RefCell is similar to Box just that for Box borrowing rules are applied at compiler time(code won't compiler if we break the rules)
    // for RefCell the rules are applied at run time(code will panic and exit if rules are broker)

    // The RefCell<T> type is useful when we’re sure that our code follows the borrowing rules but the compiler is unable to understand and guarantee that.

    // Recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

    // Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
    // Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
    // Because RefCell<T> allows mutable borrows checked at runtime, we can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

    // Mutating the value inside an immutable value is the interior mutability pattern.

    // Interior Mutability: A Mutable Borrow to an Immutable Value

    // A consequence of the borrowing rules is that when we have an immutable value, we can’t borrow it mutably. 

    // this is not possible
    // let a = 4;
    // let b = &mut a;

    // but there are situations where we want to mutate a immutable looking value
    // Example - there would be useful for a value to mutate itself in its methd but appear immutable to outside code.
    // code outside the methods of the value will not be able to mutate it
    // using RefCell is one way.

    // A practicle example
    // Test Double - programming concept for a type used in place of another type during testing.
    // Mock objects are specific type of test doubles that record what happens during a test so we can assert that correct action took place.

    // Rust don't have object we can create Struct that will serve same purpose

    // Important
    // https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#:~:text=Listing%2015%2D20%3A%20A%20library%20to%20keep%20track%20of%20how%20close%20a%20value%20is%20to%20a%20maximum%20value%20and%20warn%20when%20the%20value%20is%20at%20certain%20levels

    it_sends_an_over_75_percent_warning_message();

    // we can't do this self.sent_messages.push(String::from(message));
    // as we take a immutable reference of the message
    // we only need the mutable reference for testing purposes so changing the code to accept mutable does not make any sense

    // Here interior mutability can help
    // we will store sent_messages in RefCell<T> and then send message will be able to modify sent_message to store the new message

    // Keeping Track of Borrows at Runtime with RefCell<T>

    // for normal variables we use & for immutable references and &mut for mutable references.
    // for RefCell we use bororow() & borrow_mut() methods respectively

    // Borrow method return a smart pointer Ref<T>
    // Borrow_mut method return a smart pointer RefMut<T>
    // both have Deref and Drop trait implemented


}

use std::cell::RefCell;

struct MockMessenger {
    // sent_messages: Vec<String>,
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            // sent_messages: vec![],
            sent_messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        // self.sent_messages.push(String::from(message));
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

fn it_sends_an_over_75_percent_warning_message() {
    let mut mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mut mock_messenger, 100);

    limit_tracker.set_value(80);

    // assert_eq!(mock_messenger.sent_messages.len(), 1);
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use crate::advance_rust::smart_pointers::List::{Cons, Nil};

struct CustomPointer{
    data: String
}

impl Drop for CustomPointer {
    fn drop(&mut self){
        println!("Dropping custom pointer {}", self.data);
    }
}

struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {

    // defines an associated type for the Deref trait to use
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// enum List {
//         Cons(i32, Box<List>),
//         Nil,
//     }

// use crate::List::{Cons, Nil};
