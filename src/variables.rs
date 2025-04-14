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
    let a: String = String::from("Hola mundo");

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

    // Tipos de datos enteros en Rust

    // Declaración de un entero sin signo de 32 bits (u32)
    // Puede almacenar valores positivos desde 0 hasta 4,294,967,295
    let entero: u32 = 10;

    // Declaración de un entero sin signo de 64 bits (u64)
    // Ideal para almacenar números muy grandes, solo positivos
    let entero_largo: u64 = 10000000000; // Los guiones bajos son solo para facilitar la lectura

    // Declaración de un entero sin signo de 16 bits (u16)
    // Útil cuando sabemos que los valores serán pequeños y queremos optimizar el uso de memoria
    let entero_corto: u16 = 1000;

    // Imprimir los valores usando interpolación
    println!(
        "Entero: {}, Entero largo: {}, Entero corto: {}",
        entero, entero_largo, entero_corto
    );
    // Salida: Entero: 10, Entero largo: 10000000000, Entero corto: 1000

    // Entero sin signo (u32): solo acepta valores positivos
    let no_negativo: u32 = 100;

    // Entero con signo (i32): permite valores tanto positivos como negativos
    let si_negativo: i32 = -100;

    println!("No negativo: {}, Sí negativo: {}", no_negativo, si_negativo);
    // Salida: No negativo: 100, Sí negativo: -100


    /*
    Recomendación: Aunque Rust puede inferir automáticamente el tipo de variable, 
    es una buena práctica especificarlo tú mismo. Esto no solo mejora la legibilidad 
    del código, sino que también te permite tener un mayor control sobre el uso de memoria. 
    Además, conocer el tamaño del tipo de dato que estás utilizando puede ayudarte a evitar 
    errores de compilación y optimizar el rendimiento de tu programa. 
    */

    
}
