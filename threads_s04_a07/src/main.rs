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

É possível passar parâmetros complexos para as várias threads criadas (scoped)?

Sim.
*/

use std::thread;
use std::time::Duration;

struct Pessoa {
	nome: String,
	saldo_conta: f64,
	saldo_poupanca: f64,
}


fn thread_contadora(pessoa: &mut Pessoa) {
	pessoa.nome.push_str(" alterado");
	pessoa.saldo_conta += 0.50;
	println!("Pessoa {} tem saldo total {}",pessoa.nome, pessoa.saldo_conta+pessoa.saldo_poupanca);
	//panic!("!@#$%");
}


fn main() {
	// Valores para as threads filhas
	let mut dados = vec![
		Pessoa{ nome:"joão".to_string(), saldo_conta:100.0, saldo_poupanca:1000.0},
		Pessoa{ nome:"maria".to_string(), saldo_conta:200.0, saldo_poupanca:2000.0},
		Pessoa{ nome:"josé".to_string(), saldo_conta:300.0, saldo_poupanca:3000.0}
		];

	// Cria as threads filhas
	thread::scope( |scope| {
		let mut handles = Vec::new();

		for pessoa in dados.iter_mut() {
			handles.push( scope.spawn( || {
							thread_contadora(pessoa);
						}) );
		};

		// Espera automaticamente todas as threads filhas terminarem, ou ...
		//for h in handles.into_iter() {		// Precisa ownership, por isto 'into_iter'
		//	let _ = h.join();
		//}

	});

    for i in 1..5 {
        println!("Thread main:  Está na contagem {}",i);
        thread::sleep(Duration::from_millis(1000));
    }

	// Thread main MANTEVE a propriedade do valor indicado por 'dados'
	println!("{:?}", dados[0].nome);

	println!("Thread main: terminou !");

}


