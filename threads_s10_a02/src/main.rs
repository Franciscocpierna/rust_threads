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
Como calcular números primos de forma sequencial melhorada ?

0 e 1 não são primos, 2 é primo.
Um número natural é primo se ele é maior que 1 e 
é divisível apenas por si próprio e por 1.

Trial Division Improvments and Implementations
FELIX HEDENSTRÖM
https://www.diva-portal.org/smash/get/diva2:1127438/FULLTEXT02

*/

use std::time::Instant;


#[derive(Debug)]
struct PrimosConhecidos {
	primos: Vec<u64>,
	inicio_intervalo: u64,
	fim_intervalo: u64,
}
impl PrimosConhecidos {
	fn junta( &mut self, novos: &mut PrimosConhecidos) {
		// Precisa continuidade
		if self.fim_intervalo+1 != novos.inicio_intervalo {
			panic!("junta não encontrou continuidade");
		}
		// self.inicio_intervalo fica como está
		self.fim_intervalo = novos.fim_intervalo;
		self.primos.append(&mut novos.primos);
	}
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


// Gera um novo conjunto de números primos
fn gera_primos_v1( primos_conhecidos: &PrimosConhecidos, inicio_novo: u64, fim_novo:u64) -> PrimosConhecidos {
	// Precisa de todos os primos até a raiz quadrada do último alvo, inclusive
	if primos_conhecidos.fim_intervalo * primos_conhecidos.fim_intervalo < fim_novo {
		panic!("gera_primos_v1 chamada sem os números primos necessários");
	}

	// Não vale a pena testar alvos que já são conhecidos
	if primos_conhecidos.fim_intervalo >= inicio_novo {
		panic!("gera_primos_v1 chamada para testar intervalo já conhecido parcialmente");
	}

	let mut novos_primos =
		PrimosConhecidos {
			primos: Vec::new(),
			inicio_intervalo: inicio_novo,
			fim_intervalo: fim_novo,
		};

	for alvo in inicio_novo..fim_novo {
		if alvo % 1_000_000 == 0 {
			println!("gera_primos_v1: alvo {} ...", alvo);
		}
		let mut eh_primo = true;
		for divisor in primos_conhecidos.primos.iter() {
			if divisor*divisor > alvo {
				break;		// Primo, pois só precisa testar até a raiz quadrada do alvo
			}
			if alvo % divisor == 0 {
				eh_primo = false;		// Não é primo, divisão inteira pelo divisor
				break;
			}
		}
		if eh_primo {
			novos_primos.primos.push(alvo);
		}
	}
	novos_primos
}


fn main() {

	let mut primos_conhecidos =
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

	
	// Expande os primos conhecidos de até 10 para até 100	
	let mut novos = gera_primos_v1(&primos_conhecidos, 11, 100);
	println!("\nNovos primos gerados por gera_primos_v1: \n{:?}", novos);
	// Junta os novos aos primos conhecidos
	primos_conhecidos.junta(&mut novos);
	println!("\nPrimos conhecidos: \n{:?}", primos_conhecidos);

	// Expande os primos conhecidos de até 100 para até 10000
	let mut novos = gera_primos_v1(&primos_conhecidos, 101, 10_000);
	println!("\nNovos primos gerados por gera_primos_v1: \n{:?}", novos);
	// Junta os novos aos primos conhecidos
	primos_conhecidos.junta(&mut novos);
	println!("\nPrimos conhecidos: \n{:?}", primos_conhecidos);
	
	// Expande os primos conhecidos de até 10_000 para até 10_000_000
	let start = Instant::now();
	let mut novos = gera_primos_v1(&primos_conhecidos,
									 10_001, 10_000_000);	// Poderia ser 100_000_000
	let duration = Instant::now() - start;
	//println!("\nNovos primos gerados por gera_primos_v1: \n{:?}", novos);
	// Junta os novos aos primos conhecidos
	primos_conhecidos.junta(&mut novos);

	//println!("\nPrimos conhecidos: \n{:?}", primos_conhecidos);
	println!("\nLista completa de primos gerados: {} -> {}, exemplo qualquer {}", 
												primos_conhecidos.inicio_intervalo,
												primos_conhecidos.fim_intervalo,
												primos_conhecidos.primos[123456]);
	println!("Geração dos primos entre 10_001 e 10_000_000 demorou {} us", duration.as_micros());

}
