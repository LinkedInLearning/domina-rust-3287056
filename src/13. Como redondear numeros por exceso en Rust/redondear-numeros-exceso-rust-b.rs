/*
 * Curso: Domina Rust
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

fn main(){

    let numero_que_deseamos_redondear: f64 = 4.815;
    
    let numero_redondeado = numero_que_deseamos_redondear.ceil() as i32;
    
    println!("Número original: {}", numero_que_deseamos_redondear);
    println!("Número redondeado por exceso: {}", numero_redondeado);
}