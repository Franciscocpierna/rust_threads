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

É possível passar parâmetros complexos para as várias threads criadas (move)?

Sim.
*/

use std::thread::{self,JoinHandle};
use std::time::Duration;

struct Pessoa {
	nome: String,
	saldo_conta: f64,
	saldo_poupanca: f64,
}


fn cria_thread_contadora(mut pessoa: Pessoa) -> JoinHandle<()> {
	thread::spawn( move || {
		pessoa.nome.push_str(" alterado");
		println!("Pessoa {} tem saldo total {}",pessoa.nome,
												pessoa.saldo_conta+pessoa.saldo_poupanca);
	})
}


fn main() {
	// Valores para as threads filhas
	let dados = vec![
		Pessoa{ nome:"joão".to_string(), saldo_conta:100.0, saldo_poupanca:1000.0},
		Pessoa{ nome:"maria".to_string(), saldo_conta:200.0, saldo_poupanca:2000.0},
		Pessoa{ nome:"josé".to_string(), saldo_conta:300.0, saldo_poupanca:3000.0}
		];


	// Cria as threads filhas
	let mut handles = Vec::new();
	for pessoa in dados.into_iter() {
		handles.push( cria_thread_contadora(pessoa) );
	}
	
    for i in 1..5 {
        println!("Thread main:  Está na contagem {}",i);
        thread::sleep(Duration::from_millis(1000));
    }

	// Espera todas as threads filhas terminarem
	for h in handles.into_iter() {
		h.join().unwrap();		// Precisa ownership, por isto 'into_iter'
	}

	// Thread main perdeu a propriedade do valor indicado por 'dados'
	//println!("{:?}", dados[0].nome);

	println!("Thread main: terminou !");

}


