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

A propriedade (ownership) é transferida (movida) através do canal ?

Sim.
*/


use std::sync::mpsc;
use std::thread;

fn main() {
	let (tx, rx) = mpsc::channel();

	// Código da thread filha
	thread::spawn(move || {
		let msg = String::from("mensagem qualquer");
		println!("Thread filha: Vai enviar a mensagem '{}'", msg);
		// Perdeu a propriedade do String indicado por 'msg'
		let _ = tx.send(msg);
		//println!("Thread filha: A mensagem enviada foi '{}'", msg);
		// Perdeu a propriedade do String indicado por 'msg'
	});


	let recebida = rx.recv().unwrap();			// Recebe aqui a propriedade do String
	println!("Thread main: Recebeu '{}'", recebida);
}


