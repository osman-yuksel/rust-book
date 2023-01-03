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
    
        println!("{}, world!", s1); //error
        println!("{}, world!", s2);
        println!("{}, world!", s3); //no error
    }
    
}
