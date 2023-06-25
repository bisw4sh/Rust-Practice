use std::io;
fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");

        // This is an example of a line comment.
    // There are two slashes at the beginning of the line.
    // And nothing written after these will be read by the compiler.
    // println!("Hello, world!");

    // Run it. See? Now try deleting the two slashes, and run it again.

    /*
     * This is another type of comment, a block comment. In general,
     * line comments are the recommended comment style. But block comments
     * are extremely useful for temporarily disabling chunks of code.
     * /* Block comments can be /* nested, */ */ so it takes only a few
     * keystrokes to comment out everything in this main() function.
     * /*/*/* Try it yourself! */*/*/
     */

    /*
    Note: The previous column of `*` was entirely for style. There's
    no actual need for it.
    */

    // You can manipulate expressions more easily with block comments
    // than with line comments. Try deleting the comment delimiters
    // to change the result:
    let x: i32 = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
    let name:&str = "Biswash";
    greeting(name);
    //To get sum of two numbers
    println!("enter a number:");
    let mut stra = String::new();
    io::stdin()
        .read_line(&mut stra)
        .expect("failed to read input.");
    println!("enter b number:");
    let mut strb = String::new();
    io::stdin()
        .read_line(&mut strb)
        .expect("failed to read input.");
  
    let a: i32 =  stra.trim().parse().expect("invalid input");
    let b: i32 =  strb.trim().parse().expect("invalid input");
  
    sum(a,b);
}

fn greeting(name : &str){
    println!("Hello, {} welcome rust programming", name);
}

fn sum(x: i32, y: i32) {
    println!("sum = {}", x+y);
}