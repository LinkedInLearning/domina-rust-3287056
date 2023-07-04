/*
 * Curso: Domina Rust
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

fn main(){

    let nombre_de_usuario1 = "@EliezerLopez";
    let nombre_de_usuario2 = "LinkedIn";

    // std::cmp::Ordering
    let resultado = nombre_de_usuario1.cmp(&nombre_de_usuario2);

    if resultado == std::cmp::Ordering::Equal {
        // Son iguales
    } else {
        // Son diferentes
    }
}