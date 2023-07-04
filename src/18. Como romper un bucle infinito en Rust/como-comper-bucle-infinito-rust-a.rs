/*
 * Curso: Domina Rust
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

use std::io;

fn main() {

    loop {
        println!("Número: ");
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Error al leer la entrada");
        
        let numero: i32 = entrada.trim().parse().expect("Número no detectado");

        if numero % 2 == 0 {
            println!("El número {} es par.", numero);
        } else {
            println!("El número {} es impar.", numero);
        }
    }
}