pub fn variables_en_rust(){
    // Variables
    let x = 5; // immutable variable
    let mut y = 10; // mutable variable
    println!("x: {}, y: {}", x, y);
    
    y += 5; // modifying mutable variable
    println!("Updated y: {}", y);
    
    // Constants
    const PI: f64 = 3.14159;
    println!("PI: {}", PI);
    
    // Shadowing
    let z = 15;
    let z = z + 5; // shadowing the previous value of z
    println!("Shadowed z: {}", z);
    
    // Data types
    let a: i32 = 42; // integer
    let b: f64 = 3.14; // floating point
    let c: char = 'R'; // character
    let d: bool = true; // boolean
    
    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
    
    // Tuples
    let tuple = (1, "Hello", 3.14);
    println!("Tuple: {:?}", tuple);
    
    // Arrays
    let arr = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);

}