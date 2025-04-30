pub fn control_de_flujo(){


    // control de flojo "if"
    // aqui estamos comparando si la variable tiene_licencia es igual a true
    let tiene_licencia = true;
    // si la variable tiene_licencia es igual a true, entonces se ejecuta el bloque de codigo
    if tiene_licencia {
        println!("Juan puede conducir.");
    }

    //control de flojo "else"
    // aqui estamos definindo la edad de juan
    let edad_juan:i8 = 17;
    // aqui estamos comparando si la edad de juan es mayor a 18; es una adulto 
    if edad_juan >= 18 {
        println!("Juan es un adulto.");
    } else {
        // si la edad de juan no es mayor a 18, entonces se ejecuta el bloque de codigo
        println!("Juan no es un adulto.");
    }

}