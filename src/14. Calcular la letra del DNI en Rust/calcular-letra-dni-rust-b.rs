/*
 * Curso: Domina Rust
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

fn main(){

    // 00000000A

    // TRWAGMYFPDXBNJZSQVHLCKE
    let letras: [char; 23] = ['T', 'R', 'W', 'A', 'G', 'M', 'Y', 'F', 'P', 'D', 'X', 'B', 'N', 'J', 'Z', 'S', 'Q', 'V', 'H', 'L', 'C', 'K', 'E'];
    
    let mut numero_dni: u32 = 12345678;
    let mut posicion: usize = (numero_dni % 23) as usize;
    let mut letra: char = letras[posicion];

    println!("La letra del DNI es: {}", letra);
}