/*
 * Curso: Domina Rust
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

 fn main(){
    
    let email_formal = true;
    
    let saludo_inicial = if email_formal {"Estimado/a,"} else {"Hola,"};
    let despedida = if email_formal {"Un cordial saludo."} else {"¡Nos vemos pronto!"};

    println!("Saludo inicial: {}", saludo_inicial);
    println!("Despedida: {}", despedida);
}