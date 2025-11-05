use std::io;

/// Demostración de ownership, borrowing y clonación en Rust.
pub fn ownership() {
    // 🔹 &str es un slice de cadena — datos inmutables almacenados en el binario.
    // No se puede modificar ni reasignar porque apunta a una posición fija en memoria.
    let a: &str = "hola";
    println!("{}", a);

    // 🔹 String es un tipo heap-allocated (almacenado en el heap).
    // Puede crecer, modificarse y transferirse (move semantics).
    let b: String = String::from("hola");

    // ❌ No se puede hacer let c = b; porque eso movería el ownership de b a c.
    // 🔹 Para mantener ambos, se usa clone(), que crea una copia profunda.
    let c: String = b.clone();
    println!("{}", c);

    // 🔹 String::new() crea una cadena vacía y mutable.
    let mut name: String = String::new();

    println!("👉 Escribe tu nombre:");
    io::stdin()
        .read_line(&mut name)
        .expect("❌ Error al leer la línea");

    // 🔹 Pasamos una referencia inmutable (&String)
    // porque solo queremos leer la longitud, no modificar la cadena.
    let longitud = caracteres_en_tu_nombre(&name);

    // 🔹 Pasamos una referencia mutable (&mut String)
    // porque la función modificará la cadena.
    add_to_string(&mut name);

    println!(
        "✅ Tu nombre tiene {} caracteres y ahora es: {}",
        longitud, name
    );
}

/// 🔧 Agrega una frase al final del String recibido.
/// Usa &mut String para permitir modificar el valor original.
fn add_to_string(s: &mut String) {
    s.push_str(" - Hola mundo!");
}

/// 📏 Calcula la cantidad de caracteres en una cadena.
/// Usa &String (o mejor aún, &str) porque solo necesita leer.
fn caracteres_en_tu_nombre(s: &String) -> usize {
    s.trim().len() // trim() para eliminar el salto de línea al final
}
