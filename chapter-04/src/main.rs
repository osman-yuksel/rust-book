// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
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

    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length_noref(s1);
        println!("The length of '{}' is {}.", s2, len);
        println!("The length of '{}' is {}.", s2, calculate_length_ref(&s2));
    }

    {
        // Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
        let s = String::from("hello");

        fn change(some_string: &String) {
            //<------------------------------->
            // some_string.push_str(", world"); // error
            println!("{}", some_string)
        }
        change(&s);
    }

    {
        // We can fix the code to allow us to modify a borrowed value with just a few small tweaks that use, instead, a mutable reference:
        let mut s = String::from("hello");
    
        fn change(some_string: &mut String) {
            some_string.push_str(", world"); // no error
        }
        change(&mut s);
        println!("{}", s);
    }
    
    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);
        // variables r1 and r2 will not be used after this point
    
        let r3 = &mut s; // no problem
        // <------------------------------------------------------------>
        // We can't use 2 diffirent mutable refereneces at the same time.
        // let r4 = &mut s; // error
        println!("{}", r3);
    }
    
    {
        let reference_to_nothing = dangle();
        //this function's return type contains a borrowed value, but there is no value for it to be borrowed from
        // fn dangle() -> &String {
        //     let s = String::from("hello");
        //     &s
        // }

        fn dangle() -> String {
            let s = String::from("hello");s
        }
        println!("{}", reference_to_nothing);
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

fn calculate_length_noref(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
} // s and length returned and moves out to the calling function

fn calculate_length_ref(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped