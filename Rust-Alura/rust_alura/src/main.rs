static GLOBAL_IMUTAVEL:u8 = 1;
static mut GLOBAL_MUTAVEL:u8 = 2;

fn soma(a:i32, b:i32) -> i32
{
    println!("{} + {} = {}", a, b, a +b);
    a + b
}

fn escopo() {

    let variavel = 300;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal = 2.5;
    println!("decimal = {}, tamanho = {} bytes", decimal, std::mem::size_of_val(&decimal));

    let mut booleana: bool = false;
    println!("tamanho da booleana = {}", std::mem::size_of_val(&booleana));

    booleana = true;
    println!("o valor da booleana é = {}", booleana);

    let letra:char = 'C';
    println!("Tamanho da letra = {}", std::mem::size_of_val(&letra));

    const PI:f64 = 3.141592654;
    println!("Constante PI = {}", PI);

    println!("Global imutavel = {}", GLOBAL_IMUTAVEL);

    unsafe {
        println!("Global mutavel = {} - esse tipo de coisa não é seguro e deve ser evitado", GLOBAL_MUTAVEL);
    }

}

fn main(){
    escopo();

    println!("Soma = {}", soma(2, 5));

}