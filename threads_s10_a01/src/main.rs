/*

The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.76.0 (released 2024-02-08) or later. 
https://doc.rust-lang.org/stable/book/
https://doc.rust-lang.org/book/ch16-00-concurrency.html

Rust Atomics and Locks
Low-Level Concurrency in Practice
by Mara Bos
https://marabos.nl/atomics/
https://github.com/m-ou-se/rust-atomics-and-locks

Rust
https://www.rust-lang.org/

Learn Rust
https://www.rust-lang.org/learn

The Rust Standard Library
https://doc.rust-lang.org/stable/std/

The Cargo Book
https://doc.rust-lang.org/cargo/index.html


Nesta aula:
Como calcular números primos de forma sequencial simples ?

0 e 1 não são primos, 2 é primo.
Um número natural é primo se ele é maior que 1 e 
é divisível apenas por si próprio e por 1.

Trial Division Improvments and Implementations
FELIX HEDENSTRÖM
https://www.diva-portal.org/smash/get/diva2:1127438/FULLTEXT02

*/



#[derive(Debug)]
struct PrimosConhecidos {
	primos: Vec<u64>,
	inicio_intervalo: u64,
	fim_intervalo: u64,
}


// Retorna 'true' caso o alvo seja um número primo
fn testa_primo_v1(alvo: u64) -> bool {
	if alvo <= 1 {
		return false;
	}
	let mut divisor = 2;
	while divisor*divisor <= alvo {		// Só precisa testar até a raiz quadrada do alvo
		if alvo % divisor == 0 {
			return false;
		}
		divisor += 1;
	}
	true
}


// Retorna 'true' caso o alvo seja um número primo
fn testa_primo_v2(alvo: u64) -> bool {
	// Resolve alvos até 10 de forma direta
	if alvo <= 1 {
		return false;
	} else if alvo==2 || alvo==3 || alvo==5 || alvo==7 {
		return true;
	} else if alvo % 2 == 0 {
		return false;
	} else if alvo % 3 == 0 {
		return false;
	} else if alvo % 5 == 0 {
		return false;
	} else if alvo % 7 == 0 {
		return false;
	}
	// Alvo é maior que 10, e divisores até 10 inclusive não precisa mais testar
	let mut divisor = 11;
	while divisor*divisor <= alvo {		// Só precisa testar até a raiz quadrada do alvo
		if alvo % divisor == 0 {
			return false;
		}
		divisor += 2;	// Só precisa testar divisor impar
	}
	true
}


// Retorna 'true' caso o alvo seja um número primo
fn testa_primo_v3( primos_conhecidos: &PrimosConhecidos, alvo: u64) -> bool {
	// Precisa de todos os primos até a raiz quadrada do alvo, inclusive
	if primos_conhecidos.fim_intervalo * primos_conhecidos.fim_intervalo < alvo  ||
		primos_conhecidos.inicio_intervalo > 0 {
		panic!("testa_primo_v3 chamada sem os números primos necessários");
	}

	if alvo == 1 {
		return false;
	}

	for divisor in primos_conhecidos.primos.iter() {
		if divisor*divisor > alvo {
			break;		// Só precisa testar até a raiz quadrada do alvo
		}
		if alvo % divisor == 0 {
			return false;
		}
	}
	true
}


fn main() {
    println!("\nnumero_primo_v1({}) -> {}", 1, testa_primo_v1(1));
    println!("numero_primo_v1({}) -> {}", 2, testa_primo_v1(2));
    println!("numero_primo_v1({}) -> {}", 3, testa_primo_v1(3));
    println!("numero_primo_v1({}) -> {}", 8, testa_primo_v1(8));
    println!("numero_primo_v1({}) -> {}", 97, testa_primo_v1(97));

    println!("\nnumero_primo_v2({}) -> {}", 1, testa_primo_v2(1));
    println!("numero_primo_v2({}) -> {}", 2, testa_primo_v2(2));
    println!("numero_primo_v2({}) -> {}", 3, testa_primo_v2(3));
    println!("numero_primo_v2({}) -> {}", 8, testa_primo_v2(8));
    println!("numero_primo_v2({}) -> {}", 97, testa_primo_v2(97));

	let primos_conhecidos =
		PrimosConhecidos {
			primos: vec![2,3,5,7],
			inicio_intervalo: 0,
			fim_intervalo: 10,
		};

    println!("\nnumero_primo_v3({}) -> {}", 1, testa_primo_v3(&primos_conhecidos,1));
    println!("numero_primo_v3({}) -> {}", 2, testa_primo_v3(&primos_conhecidos,2));
    println!("numero_primo_v3({}) -> {}", 3, testa_primo_v3(&primos_conhecidos,3));
    println!("numero_primo_v3({}) -> {}", 8, testa_primo_v3(&primos_conhecidos,8));
    println!("numero_primo_v3({}) -> {}", 97, testa_primo_v3(&primos_conhecidos,97));

}
