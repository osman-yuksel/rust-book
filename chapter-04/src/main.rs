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
    
        //println!("{}, world!", s1); // error
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
    
        // x comes into scope
        let x = 5;
        // x would move into the function, but i32 is Copy, so it's okay to still use x afterward
        makes_copy(x);
    }
    
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.