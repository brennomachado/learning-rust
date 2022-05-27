use std::mem;

fn main() {

    let variavel = 300;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal = 2.5;
    println!("decimal = {}, tamanho = {} bytes", decimal, std::mem::size_of_val(&decimal));

    let mut booleana: bool = false;
    println!("tamanho da booleana = {}", std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("Tamanho da letra = {}", std::mem::size_of_val(&letra));
}
