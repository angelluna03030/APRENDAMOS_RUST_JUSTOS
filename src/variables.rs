pub fn variables_en_rust() {
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

    // String en Rust

    // Primera forma de declarar una cadena de texto:
    // Usando String::from para crear un String (tipo dinámico y mutable)
    let a:String = String::from("Hola mundo");

    // Segunda forma de declarar una cadena:
    // Usando un *slice* de cadena (&str), que es una referencia a un texto fijo en memoria
    let b: &str = "Hola mundo";

    // Imprimiendo las variables
    println!("a: {}", a); // Imprime: a: Hola mundo,

    // También se puede imprimir solo una variable
    println!("b: {}", b); // Imprime: b: Hola mundo




    // Para mostrar resultados en la consola en Rust,
    // se utiliza la macro `println!`.
    // Esta macro permite imprimir texto y variables.
    // Se usan llaves `{}` como marcadores de posición
    // para insertar los valores de las variables dentro del texto.
    let nombre = "Carlos";
    println!("Hola, {}!", nombre); // Imprime: Hola, Carlos!





    
}
