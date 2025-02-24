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

O que acontece no 'join' se a thread criada entra em pânico ?

'join' retorna Result::Err.
*/

use std::thread;
use std::time::Duration;

fn main() {
	let handle = thread::spawn(|| {
		let mut lista = Vec::new();
		lista.push(0);
		for i in 1..10 {
			lista.push(i);
			println!("Thread filha:  está na contagem {}",i);
			thread::sleep(Duration::from_millis(1));
		}
		lista[2]
		//lista[20]		// Esta linha força um pânico na thread filha
		//lista			// Retorna a lista inteira
	});

	for i in 1..5 {
		println!("Thread main:  está na contagem {}",i);
		thread::sleep(Duration::from_millis(1));
    }

	if let Ok(x) = handle.join() {
		println!("Thread main:  filha terminou, retornou '{}'", x);
	} else {
		println!("Thread main:  filha terminou com pânico");
	}

	// 3 opções:
	// if let Ok(x) ...					Trata a situação de pânico na thread filha 
	// handle.join().unwrap();			Main entra em pânico se thread filha retornar Err (caso ela aborte)
	// let _ = handle.join();			Main ignora retorno do 'join' e continua sua execução

	println!("Thread main:  terminou normalmente");

}




