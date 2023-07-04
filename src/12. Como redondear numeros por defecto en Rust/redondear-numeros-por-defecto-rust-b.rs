/*
 * Curso: Domina Rust
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

fn main(){

    let numero_que_deseamos_redondear: f64 = 4.81516;
    
    let numero_redondeado = numero_que_deseamos_redondear.floor() as i32;
    
    println!("Número original: {}", numero_que_deseamos_redondear);
    println!("Número redondeado por defecto: {}", numero_redondeado);
}