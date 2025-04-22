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

    // Tipos de datos de punto flotante en Rust
    // Declaración de un número de punto flotante de 32 bits (f32)
    // Ideal para cálculos que requieren menos precisión y ocupan menos memoria
    let flotante: f32 = 3.14; // Número de punto flotante de 32 bits
    println!("Flotante: {}", flotante); // Salida: Flotante: 3.14
                                        // Declaración de un número de punto flotante de 64 bits (f64)
                                        // Ideal para cálculos que requieren mayor precisión
    let flotante_largo: f64 = 3.14159265358979323846;
    // Número de punto flotante de 64 bits
    println!("Flotante largo: {}", flotante_largo);
    // Salida: Flotante largo: 3.14159265358979323846

    // Tipos de datos boolean en Rust
    // let _verdadero: bool = true;
    // let verdadero = bool::from(true); // Booleano verdadero
    // let falso: bool = false;
    // let _falso1 = bool::from(false); // Booleano falso
    // println!("Verdadero: {}, Falso: {}", _verdadero);
    // // Salida: Verdadero: true
    // println!("Falso: {}", falso);
    // Salida: Falso: false

    // Tipos de datos tuplas en Rust

    // Tupla con tres tipos de datos diferentes
    let tupla: (i32, f64, char) = (42, 3.14, 'R');
    println!("Tupla: ({}, {}, {})", tupla.0, tupla.1, tupla.2);
    // salida : Tupla: (42, 3.14, R)
    // Desestructuración de tuplas
    let (x, y, z) = tupla; // Asignación de valores de la tupla a variables
    println!("Desestructurada: x: {}, y: {}, z: {}", x, y, z);
    // salida : Desestructurada: x: 42, y: 3.14, z: R
    // Acceso a elementos de la tupla
    println!("Acceso: x: {}, y: {}, z: {}", tupla.0, tupla.1, tupla.2);
    // salida : Acceso: x: 42, y: 3.14, z: R
    // Acceso a elementos de la tupla usando
    println!("Acceso: {:?}", tupla);
    // salida : Tupla: (42, 3.14, R)

    // Tipos de datos arreglos en Rust


    // Arreglo de enteros de tamaño fijo de 5
    // El tamaño del arreglo es fijo y se define en el momento de la declaración
    // El tipo de dato del arreglo es i32 (entero de 32 bits)
    // El arreglo contiene 5 elementos, todos inicializados a 0
    let arreglo : [i32; 5]   = [1, 2, 3, 4, 5];
    // Arreglo de enteros de tamaño fijo
    println!("Arreglo: {:?}", arreglo);
    // salida : Arreglo: [1, 2, 3, 4, 5]
    // Acceso a elementos del arreglo
    println!("Acceso: {}, {}, {}", arreglo[0], arreglo[1], arreglo[2]);
    // salida : Acceso: 1, 2, 3
    // Un arreglo de 10 elementos, todos inicializados a 0
    let arreglo = [0; 10];
    println!("Arreglo: {:?}", arreglo);
    // salida : Arreglo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
     
    // Imprime el arreglo completo
    /*
    Recomendación: Aunque Rust puede inferir automáticamente el tipo de variable,
    es una buena práctica especificarlo tú mismo. Esto no solo mejora la legibilidad
    del código, sino que también te permite tener un mayor control sobre el uso de memoria.
    Además, conocer el tamaño del tipo de dato que estás utilizando puede ayudarte a evitar
    errores de compilación y optimizar el rendimiento de tu programa.
    */



    // Esta macro permite imprimir estructuras como tuplas y arreglos.
    // Para imprimir estos tipos, se usa el marcador de posición `{:?}`,
    // que muestra los valores de manera completa y en formato de depuración.
    let mi_tupla= (42, "Hola", true);
    let mi_arreglo = [1, 2, 3, 4, 5];
    println!("Tupla: {:?}", mi_tupla);
     // Imprime: Tupla: (42, "Hola", true)
    println!("Arreglo: {:?}", mi_arreglo); 
    // Imprime: Arreglo: [1, 2, 3, 4, 5]

    // Tipo de varible Inmutable 
    let inmutable = 10; // variable inmutable
    //inmutable = 20; // Esto causará un error de compilación porque la variable es inmutable
    let inmutable2 = 20; // variable inmutable
    const PI2: f64 = 3.14159; // constante inmutable
    println!("Inmutable: {}", inmutable); // Imprime: Inmutable: 10
    println!("Inmutable: {}", inmutable2); // Imprime: Inmutable: 20
    println!("Constante: {}", PI2); // Imprime: Constante: 3.14159

    // Tipo de varible Mutable
    let mut mutable = 10; // variable mutable
    println!("Initial Mutable: {}", mutable); // Imprime: Initial Mutable: 10
    mutable = 20; // Esto está permitido porque la variable es mutable
    println!("Mutable: {}", mutable); // Imprime: Mutable: 20
    mutable = 30; // Esto también está permitido porque la variable es mutable
    println!("Mutable: {}", mutable); // Imprime: Mutable: 30

    


}
