
use configs::menu::Menu;

pub fn Operacoes() {
    
        let mut operacao: String = String::new();
                io::stdin().read_line(&mut operacao).expect("Erro linha");

                // Remove a quebra de linha e converte 
                operacao = operacao.trim().to_string();

                println!("Digite o primeiro número: ");
                let mut num1: String = String::new();
                io::stdin().read_line(&mut num1).expect("Erro linha");

                println!("Digite o segundo número: ");
                let mut num2: String = String::new();
                io::stdin().read_line(&mut num2).expect("Erro linha");

                // Conversão de &str para i32
                let a: i32 = num1.trim().parse().expect("Erro conversão");
                let b: i32 = num2.trim().parse().expect("Erro conversão");
                let mut resultado: i32 = 0;
                
        if operacao == "+" {
                resultado = a + b;
            } else if operacao == "-" {
                resultado = a - b;
            } else if operacao == "/" {
                resultado = a / b;
            } else if operacao == "*" {
                resultado = a * b;
            } else if operacao == "sair" {
                println!("Saindo");
                break;
            } else {
                println!("Opção inválida");
                continue;
            }
    }