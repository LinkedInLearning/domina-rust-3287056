/*
 * Curso: Domina Rust
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

use std::f64;

fn main(){

    let numero = 16;
    let raiz_cuadrada = f64::sqrt(numero as f64);

    println!("La raíz cuadrada de {} es {}.", numero, raiz_cuadrada);
}