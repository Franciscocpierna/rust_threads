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

Uma thread filha pode acessar livremente variáveis do escopo da thread main ?

Não. Se a thread filha termina enquanto vv1 existe então estaria tudo bem.
Mas se vv1 é destruído antes da thread filha terminar
a referência que a filha tem ficaria inválida.
*/


use std::thread::{self, JoinHandle};

fn cria_thread_filha() -> JoinHandle<()> {
	let vv1 = vec![1, 2, 3];
	let vv2 = 0;		// E se for um tipo primitivo ?
	
    let handle = thread::spawn(|| {
		println!("Thread filha executa");
		println!("Aqui está vv1: {:?}", vv1 );		// Borrow implícito, closure pode sobreviver à main
		println!("Aqui está vv2: {:?}", vv2 );		// Borrow implícito, closure pode sobreviver à main
    });

	handle
}



fn main() {
	let handle = cria_thread_filha();
	handle.join().unwrap();
	println!("Thread main terminou");
}


