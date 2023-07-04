/*
 * Curso: Domina Rust
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

use rand::Rng;

fn main(){

    let mut instancia = rand::thread_rng();

    let numero_aleatorio: i32 = instancia.gen();
}