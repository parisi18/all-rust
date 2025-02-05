const PI: f32 = 3.14;
static mut GLOBAL_VAR: u8 = 1; //É unsafe criar var global mutável

fn soma(a: i32, b: i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    a + b
}

fn sombra() {
    let a = 123;

    {
        let b = 456;
        let a = 777; //Shadowing
        println!("Dentro. b = {}", b);
        println!("Dentro. a = {}", a);
    }

    println!("Fora. a = {}", a);
}

fn scope() {
    unsafe {
        println!("GLOBAL_VAR = {}", GLOBAL_VAR);
    }
    println!("PI = {}", PI); //Resolvido em tempo de compilação

    //let string = "meu nome"; //&'static str

    let var: i32 = 128;
    println!(
        "variavel = {}, tamanho = {}",
        var,
        std::mem::size_of_val(&var) //Resolvido em tempo de execução
    );

    let decimal = 2.5;
    println!("decimal = {}", decimal);

    let boolean = false;
    println!(
        "Boolean = {}, Tamanho bool = {}",
        boolean,
        std::mem::size_of_val(&boolean)
    );

    let ch = 'c';
    println!("Tamanho do char = {}", std::mem::size_of_val(&ch));
}

fn condicionais() {
    let idade: u8 = 17;
    let autorizado = true;
    let condicao;
    let maior_de_idade = idade >= 18;

    if maior_de_idade {
        println!("Pode entrar na balada");
    } else if idade > 16 && autorizado {
        println!("Pode entrar com assinatura do responsável");
    } else {
        println!("Não pode entrar na balada");
    }

    condicao = if maior_de_idade { "maior" } else { "menor" };

    println!("É {} de idade", condicao);
}

fn repeticoes() {
    let multiplicador: u8 = 5;

    let mut contador = 1;

    println!("While");
    while contador <= 10 {
        println!(
            "{} x {} = {}",
            multiplicador,
            contador,
            multiplicador * contador
        );
        contador += 1;
    }

    contador = 0;

    println!("Loop");
    loop {
        contador += 1;
        println!(
            "{} x {} = {}",
            multiplicador,
            contador,
            multiplicador * contador
        );

        if contador == 10 {
            break;
        }

        if contador == 5 {
            continue;
        }
    }

    println!("For loop");
    //[1..11[
    for contador in 1..11 {
        println!(
            "{} x {} = {}",
            multiplicador,
            contador,
            multiplicador * contador
        );
    }
}

fn ownership() {
    let mut string = String::from("Rafael"); //Ponteiro para o espaco de memoria na heap

    ref_rouba(&mut string); //Borrowship
                            //rouba(string); //Similar ao move semantics

    //println!("{}", string);
}

//Owner
fn rouba(string: String) {
    println!("{}", string);
}

//Borrow
fn ref_rouba(string: &mut String) {
    string.push_str(" Parisi");
    println!("{}", string);
}

fn pattern_matching() {
    for x in 1..=20 {
        println!(
            "{}: {}",
            x,
            match x {
                1 => "Pouco",
                2 | 3 => "Um pouquinho",
                4..=10 => "Um bocado",
                _ if x % 2 == 0 => "Uma boa quantidade",
                _ => "Muito",
            }
        )
    }
}

fn erros() {
    match result() {
        Ok(s) => println!("String de sucesso = {}", s),
        Err(numero) => println!("Codigo de erro = {}", numero),
    };
}

fn result() -> Result<String, u8> {
    let select = false;

    match select {
        true => Ok(String::from("Sucesso!")),
        false => Err(42),
    }
}

fn main() {
    scope();
    sombra();
    println!("Soma = {}", soma(1, 1));
    condicionais();
    repeticoes();
    ownership();
    pattern_matching();
    erros();
}
