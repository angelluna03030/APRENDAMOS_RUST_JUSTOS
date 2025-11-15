

pub fn structs(){
let persona: Persona = Persona { nombre: "juan".to_owned(), edad: 12, genero: "Masculino".to_owned() };
if es_hombre(&persona.genero){
    println!("La persona es un hombre");
}else {
    println!("La persona es una mujer");
}

println!("La persona se llama {}, tiene {} años y su género es {}", persona.nombre, persona.edad, persona.genero);
}

//las estruturas en rust son similares a las clases en otros lenguajes de programacion
//pero son mutables por defecto
//y no tienen metodos como en otros lenguajes de programacion
//pero se pueden implementar metodos para las estructuras
//una estructura se define con la palabra clave struct
struct Persona{
nombre: String,
edad: u8,
genero: String,

}

fn es_hombre(genero: &String) -> bool {
    genero == "Masculino"
}
