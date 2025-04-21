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
    
    
}