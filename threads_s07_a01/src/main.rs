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

O que é o Mutex e como funciona ?

Ele permite o acesso exclusivo a um valor (Mutex= Mutual Exclusion).
*/


use std::sync::Mutex;

#[derive(Debug)]
struct Pessoa {
	nome: String,
	saldo: f64,
}
impl Pessoa {
	fn new() -> Pessoa {
		Pessoa {
			nome: "".to_string(),
			saldo: 0.0,
		}
	}
}



fn main() {
	let pessoa = Mutex::new( Pessoa::new() );	// O Mutex protege um certo valor
	println!("{:?}", pessoa);

	{
		// Para alterar o valor é necessário obter o lock do Mutex
		let mut p = pessoa.lock().unwrap();
		p.nome = "Rômulo".to_string();
		p.saldo = 123.4;
		println!("Dado depois da alteração: '{}' '{}'", p.nome, p.saldo);
		// Não existe unlock explícito
		// Unlock implícito no final do escopo

		//let _pp = pessoa.lock().unwrap();		// Deadlock, mas depende da implementação
	}

	let p2 = pessoa.lock().unwrap();
	println!("Novamente dado sendo acessado: '{}' '{}'", p2.nome, p2.saldo);

	println!("{:?}", pessoa);
}



