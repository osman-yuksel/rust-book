// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
fn main() {
    {
        let mut s = String::from("hello"); // Creating a String from a string literal using the from function
    
        s.push_str(", world!"); // push_str() appends a literal to a String
    
        println!("{}", s); // This will print `hello, world!`
    } 
    // print!("{}", s); no longer valid


    // To ensure memory safety, after the line let s2 = s1;, 
    // Rust considers s1 as no longer valid. 
    // Therefore, Rust doesn't need to free anything when s1 goes out of scope. 
    // Check out what happens when you try to use s1 after s2 is created; it won't work:
    {
        let s1 = String::from("hello");
        let s2 = s1;
        let s3 = s2.clone(); 
        
        //<----------------------------------->
        // println!("{}, world!", s1); // error
        println!("{}, world!", s2);
        println!("{}, world!", s3); // no error
    }
    

    // Types such as integers that have a known size at compile time are stored entirely on the stack,
    // so copies of the actual values are quick to make.
    {
        let x = 5;
        let y = x;
        
        println!("x = {}, y = {}", x, y);
    }

    {
        // s comes into scope
        let s = String::from("hello");
        // s's value moves into the function and so is no longer valid here
        takes_ownership(s);
        //<------------------------>
        // print!("{}", s); // error


        // x comes into scope
        let x = 15;
        // x would move into the function, but i32 is Copy, so it's okay to still use x afterward
        makes_copy(x);
        println!("{}", x); // no error
    }

    {
        let s1 = gives_ownership(); 
        let s2 = String::from("hello"); 
        let s3 = takes_and_gives_back(s2);
        
        println!("{}", s1); 
        //<------------------------->
        // print!("{}", s2); // error 
        println!("{}", s3); // new owner of s2
    }
    
}

// some_string comes into scope
fn takes_ownership(some_string: String) { 
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

// some_integer comes into scope
fn makes_copy(some_integer: i32) { 
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// gives_ownership will move its return value into the function that calls it
fn gives_ownership() -> String { 
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
// a_string comes into scope
fn takes_and_gives_back(a_string: String) -> String { 
    a_string  // a_string is returned and moves out to the calling function
}