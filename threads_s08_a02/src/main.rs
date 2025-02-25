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
Como funciona o Arc (Atomic Reference Counting) ?

É uma referência para um dado cujo número de clones é contado.

Arc - Atomically Reference Counted
	'Reference' -> É uma referência para um valor
	'Counted' -> O número de referências existentes é contado
				A cada clone() o contador é incrementado
				A cada drop (destruição do clone) o contador é decrementado
				Quando o contador chega a zero a memória é liberada
	'Atomically' -> Várias threads podem clonar/dropar sem problema
*/


use std::sync::Arc;


#[derive(Debug)]
struct Pessoa {
	nome: String,
	saldo: f64,
}
impl Pessoa {
	fn new(nome:String) -> Pessoa {
		Pessoa {
			nome,
			saldo: 0.0,
		}
	}
}


fn main() {
	let original = Pessoa::new("rômulo".to_string());
	let mut pessoa = Arc::new(original);
	//println!("Original: {:?}", original);				// Original foi movido

	// Existe 1 clone do Arc
	println!("(1) {:?}", pessoa);
	println!("(1) Número de cópias existentes: {}", Arc::strong_count(&pessoa));
	println!("(1) nome: {}   saldo: {}", (*pessoa).nome, pessoa.saldo);			// Deref está implementado


	// Quando existe apenas 1 clone a função Arc::get_mut() pode ser usada, retorna 'Some'
	if let Some(p1) = Arc::get_mut(&mut pessoa) {
		p1.nome.push('X');
		p1.saldo = 222.2;
		println!("\n(2) Some -> Número de cópias existentes: {}", Arc::strong_count(&pessoa));
		println!("(2) {:?}", pessoa);
	} else {
		println!("\n(2) None -> Número de cópias existentes: {}", Arc::strong_count(&pessoa));
	}


	// Pode-se clonar várias vezes
	let pessoa_1 = pessoa.clone();
	let pessoa_2 = pessoa.clone();
	let pessoa_3 = pessoa.clone();
	println!("\n(3) pessoa_1: {:?}", pessoa_1);
	println!("(3) pessoa_2: {:?}", pessoa_2);
	println!("(3) pessoa_3: {:?}", pessoa_3);
	println!("(3) Número de cópias existentes: {}", Arc::strong_count(&pessoa));


	// Quando existem mais de 1 clone a função Arc::get_mut() retorna 'None'
	// Usaremos Mutex para acessar o conteúdo do Arc nas próximas aulas
	if let Some(p2) = Arc::get_mut(&mut pessoa) {
		p2.nome.push('Y');
		p2.saldo = 444.4;
		println!("\n(4) Some -> Número de cópias existentes: {}", Arc::strong_count(&pessoa));
		println!("(4){:?}", pessoa);
	} else {
		println!("\n(4) None -> Número de cópias existentes: {}", Arc::strong_count(&pessoa));
	}


	// Contagem é a mesma para todos os clones
	if  Arc::strong_count(&pessoa) == 4 {
		let pessoa_4 = Arc::clone(&pessoa);
		println!("\n(5) {:?}", pessoa_4);
		println!("(5) Número de cópias existentes: {} {}",
							Arc::strong_count(&pessoa), Arc::strong_count(&pessoa_4));
	}

	println!("\n(6) Drop, Número de cópias existentes: {} {}",
					Arc::strong_count(&pessoa), Arc::strong_count(&pessoa_1));


	println!("\nThread main: Terminou");
}



