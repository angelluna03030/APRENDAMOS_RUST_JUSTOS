pub fn control_de_flujo() {
    // Control de flujo en Rust
    // En Rust, el control de flujo se refiere a la forma en que se ejecutan las instrucciones en un programa.
    // Esto incluye estructuras como condicionales (if, else, else if) y bucles (for, while).
    // Estas estructuras permiten tomar decisiones y repetir acciones en función de ciertas condiciones.
    // En este ejemplo, se muestran algunos ejemplos de control de flujo en Rust.

    // control de flujo "if"
    // aqui estamos comparando si la variable tiene_licencia es igual a true
    let tiene_licencia = true;
    // si la variable tiene_licencia es igual a true, entonces se ejecuta el bloque de codigo
    if tiene_licencia {
        println!("Juan puede conducir.");
    }

    //control de flujo "else"
    // aqui estamos definindo la edad de juan
    let edad_juan: i8 = 17;
    // aqui estamos comparando si la edad de juan es mayor a 18; es una adulto
    if edad_juan >= 18 {
        println!("Juan es un adulto.");
    } else {
        // si la edad de juan no es mayor a 18, entonces se ejecuta el bloque de codigo
        println!("Juan no es un adulto.");
    }
    // salida : Juan no es un adulto.

    //control de flujo "else if"
    let edad:i8 = 18;
    // aqui estamos comparando si la edad de juan es menor a 18; es un adolescente
    if edad < 18 {
        // si la edad de juan es menor a 18, entonces se ejecuta el bloque de codigo
        println!("Juan es menor de edad.");
    } else if edad == 18 {
        // si la edad de juan es igual a 18, entonces se ejecuta el bloque de codigo
        println!("Juan acaba de ser adulto.");
    } else {
        // si la edad de juan no es menor a 18, entonces se ejecuta el bloque de codigo
        println!("Juan es mayor de edad.");
    }
    // salida : Juan acaba de ser adulto.
 


    //control de flujo "while"
    // aqui estamos definiendo la variable contador y la inicializamos en 0
    let mut contador:i8 = 15;
    while contador > 9 {
        // aqui estamos imprimiendo el valor de contador
        println!("Contar hasta que sea 10: {}", contador);
        // aqui estamos decrementando el valor de contador en 1
        contador -= 1;
    }
    // salida : Contar hasta que sea 10: 15



    //control de flujo "loop"
    // aqui estamos definiendo la variable contador y la inicializamos en 0
    let mut contador:i8 = 10;
    loop{
        println!("Contar hasta que sea 0: {}", contador);
        // aqui estamos decrementando el valor de contador en 1
        contador -= 1;
        if contador == 0 {
            // aqui estamos saliendo del bucle
            break;
        }
    }
  // aqui no se esta controlado el flujo del programa, por lo que el bucle se ejecutara indefinidamente

        // let mut contador = 1;
        
        // loop {
        //     println!("Vuelta número: {}", contador);
        //     contador += 1;
            
        //     // Nota: no hay ninguna condición que detenga el bucle
        // }
    
 
    let mut contador = 0;
    
    let resultado = loop {
        contador += 1;
        
        if contador == 10 {
            break contador * 2;  // Aquí devolvemos un valor (20)
        }
    };
    
    println!("El resultado final es: {}", resultado);  
    // Imprimirá "El resultado final es: 20"


    //control de flujo "for"
    // aqui estamos definiendo un rango de 1 a 10
    let rango = 1..10;
    // aqui estamos iterando sobre el rango
    for i in rango {
        // aqui estamos imprimiendo el valor de i
        println!("Valor de i: {}", i);
    }
    // salida : Valor de i: 1



       // control de flujo "Scope" 
       //"Scope" principal
       let x = 10;
       {
           // Un scope interno (anidado)
           let y = 5;
           println!("Dentro del scope interno: x = {}, y = {}", x, y);
           {// Un scope aún más interno
               let z = 3;
               println!("En el scope más profundo: x = {}, y = {}, z = {}", x, y, z);
           }
           // Aquí z ya no existe
           // println!("z = {}", z); // Esto causaría un error
       }
       // Aquí y ya no existe
       println!("En el scope principal: x = {}", x);
       // println!("y = {}", y); // Esto causaría un erro








    // sombreado en Rust  
    // Scope principal
    let mensaje = "Hola";
    println!("Variable original: {}", mensaje);
    {
        // Scope interno - sombreamos la variable mensaje
        let mensaje = "Saludos desde el scope interno";
        println!("Variable sombreada: {}", mensaje);
        {
            // Podemos seguir sombreando en scopes más profundos
            let mensaje = 42; // Incluso podemos cambiar el tipo
            println!("Variable doblemente sombreada (ahora es un número): {}", mensaje);
        }
        // Aquí volvemos a la primera sombra
        println!("De nuevo en el primer nivel de sombra: {}", mensaje);
    }
    // Y aquí tenemos la variable original, intacta
    println!("Variable original al final: {}", mensaje);



    // control de flujo "match"
    let calificacion = 85;
    let resultado: &str = match calificacion {
        90..=100 => "Excelente",
        80..=89 => "Muy bueno",
        70..=79 => "Bueno",
        60..=69 => "Satisfactorio",
        0..=59 => "Necesita mejorar",
        _ => "Calificación inválida" // El comodín _ captura cualquier otro valor
    };

    //curiosidad : el operador de rango ..= se utiliza para crear un rango inclusivo
    println!("Tu desempeño fue: {}", resultado); // Imprimirá "Tu desempeño fue: Muy bueno"
     // Este bucle imprimirá los números 1, 2, 3, 4, 5, 6, 7, 8, 9 y 10
     for numero in 1..=10 {
        println!("{}", numero);
    }
}





