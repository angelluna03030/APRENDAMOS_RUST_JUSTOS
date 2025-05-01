pub fn control_de_flujo() {
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

}




