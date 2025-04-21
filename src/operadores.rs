pub fn operadores_en_rust (){

 
    // Operador	Descripción	Ejemplo
    // &&	AND lógico	true && false
    // `		`
    // !	NOT lógico (negación)	!true
    // ➕ 2. Operadores Aritméticos:
    
    // Operador	Descripción	Ejemplo
    // +	Suma	5 + 3
    // -	Resta	5 - 3
    // *	Multiplicación	5 * 3
    // /	División	5 / 2
    // %	Módulo (residuo)	5 % 2
    // ⚖️ 3. Operadores de Comparación (devuelven bool):
    
    // Operador	Descripción	Ejemplo
    // ==	Igual a	5 == 5
    // !=	Distinto de	5 != 3
    // >	Mayor que	5 > 3
    // <	Menor que	5 < 3
    // >=	Mayor o igual que	5 >= 5
    // <=	Menor o igual que	5 <= 6
    // 🧠 4. Operadores Bit a Bit:
    
    // Operador	Nombre	Ejemplo
    // &	AND bit a bit	a & b
    // `	`	OR bit a bit
    // ^	XOR bit a bit	a ^ b
    // <<	Desplazamiento izq.	a << 1
    // >>	Desplazamiento der.	a >> 1
    // !	NOT bit a bit (solo para !bool)	!a
    // 📦 5. Operadores de Asignación:
    
    // Operador	Descripción	Ejemplo
    // =	Asignación	x = 5
    // +=	Suma y asigna	x += 1
    // -=	Resta y asigna	x -= 2
    // *=	Multiplica y asigna	x *= 3
    // /=	Divide y asigna	x /= 4
    // %=	Módulo y asigna	x %= 2
    // &= `	= ^= <<= >>=`	Asignaciones bit a bit

   // ✅ 1. Operadores Lógicos :


   //Operador logico &&
   let x:bool = true;
   let y:bool = bool::from(true);
   let z = false;
   println!( "x && y = {}", x && y); // true && true = true
   println!( "x && false = {}", x && false); // true && false = false
   println!( "false && y = {}", x && z); // false && true = false

   
   //Operador logico ||
   let a:bool = true;
   let b:bool = bool::from(false);
   let c= false;
   println!( "a || b = {}", a || b); // true || false = true
   println!( "a || false = {}", a || false); // true || false = true
   println!( "false || b = {}", c || b); // false || true = true
   println!( "false || false = {}", c || b); // false || false = false
   // Operador logico !
    let d:bool = true;
    let e:bool = bool::from(false);
    println!( "d = {}", d); // true
    println!( "e = {}", e); // false
    println!( "!d = {}", !d); // !true = false
    println!( "!e = {}", !e); // !false = true



    println!( "!(d && e) = {}", !(d && e)); // !(true && false) = true
    println!( "!(d || e) = {}", !(d || e)); // !(true || false) = false
    println!( "!(d && !e) = {}", !(d && !e)); // !(true && true) = false
    println!( "!(d || !e) = {}", !(d || !e)); // !(true || true) = false
    

    //2. Operadores Aritméticos:

    // Operador Aritmético +
    let suma_enteros = 5 + 3; // Suma de dos números
    println!("Suma: {}", suma_enteros); // Salida: Suma: 8
    let suma_flotantes = 5.0 + 3.0; // Suma de dos números flotantes
    println!("Suma flotantes: {}", suma_flotantes); // Salida: Suma flotantes: 8.0
    let suma_strings = String::from("Hola") + " Mundo"; // Concatenación de cadenas
    println!("Suma strings: {}", suma_strings); // Salida: Suma strings: Hola Mundo
    

    //podemos ver que aqui muestra un error en la suma    |
    //                                                    v
    //let suma_mixta_enteros = 5 + 3.0;
     // Suma de un entero y un flotante (se convierte a flotante)
    // Convertimos los booleanos a enteros antes de sumar
    let suma_mixta_bolenoas = (true as i32) + (false as i32); 
    println!("Suma mixta: {}", suma_mixta_bolenoas); // Salida: Suma mixta: 1

    // Operador Aritmético -
    let resta_enteros = 5 - 3; // Resta de dos números
    println!("Resta: {}", resta_enteros); // Salida: Resta: 2
    let resta_flotantes = 5.0 - 3.0; // Resta de dos números flotantes
    println!("Resta flotantes: {}", resta_flotantes); // Salida: Resta flotantes: 2.0
    

    // let resta_strings=String::from("Hola")-String::from(" Mundo"); // Error: no se puede restar cadenas
    // // Error: no se puede restar booleanos
    // let resta_boolenos =true-false; 
    
    // Convertimos el entero a flotante para realizar la resta
    let restar_mixta_flotantes = (5 as f32) - 3.0; 
    println!("Resta mixta: {}", restar_mixta_flotantes); // Salida: Resta mixta: 2.0
    // Resta de un entero y un flotante (se convierte a entero)
    let restar_mixta_enteros = 5 - (3.0 as u16);
    println!("Resta mixta: {}", restar_mixta_enteros); // Salida: Resta mixta: 2
    

    // Operador Aritmético *
    let multiplicacion_enteros = 5 * 3; // Multiplicación de dos números
    println!("Multiplicación: {}", multiplicacion_enteros); // Salida: Multiplicación: 15
    let multiplicacion_flotantes = 5.0 * 3.0; // Multiplicación de dos números flotantes
    println!("Multiplicación flotantes: {}", multiplicacion_flotantes); 
    // Salida: Multiplicación flotantes: 15.0

    // Multiplicación de un entero y un flotante (se convierte a flotante)
    let multiplicacion_mixta_flotantes = (5 as f32) * 3.0;
    println!("Multiplicación mixta: {}", multiplicacion_mixta_flotantes); 
    // Salida: Multiplicación mixta: 15.0

    // Multiplicación de un entero y un flotante (se convierte a entero)
    let multiplicacion_mixta_enteros = 5 * (3.0 as u16);
    println!("Multiplicación mixta: {}", multiplicacion_mixta_enteros);
     // Salida: Multiplicación mixta: 15

    // Multiplicación de un booleano y un entero (se convierte a entero)
    let multiplicacion_mixta_boolenos = (true as i32) * 3;
    println!("Multiplicación mixta: {}", multiplicacion_mixta_boolenos); 
    // Salida: Multiplicación mixta: 3
  




    // Operador Aritmético /
    let division_enteros = 5 / 2; // División de dos números enteros (entero)
    println!("División: {}", division_enteros); // Salida: División: 2
    let division_flotantes = 5.0 / 2.0; // División de dos números flotantes (flotante)
    println!("División flotantes: {}", division_flotantes); // Salida: División flotantes: 2.5
    // División de un entero y un flotante (se convierte a flotante)
    let division_mixta_flotantes = (5 as f32) / 2.0;
    println!("División mixta: {}", division_mixta_flotantes); // Salida: División mixta: 2.5
    // División de un entero y un flotante (se convierte a entero)
    let division_mixta_enteros = 5 / (2.0 as u16);
    println!("División mixta: {}", division_mixta_enteros); // Salida: División mixta: 2
    

}