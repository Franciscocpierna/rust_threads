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
Como funcionam as 'scoped threads' ?

Elas ficam contidas em um escopo (scope).
*/

use std::thread;
use std::time::Duration;


fn main() {

	thread::scope( |scope| {

	 	// Cria thread AAA
		//let handle =
		scope.spawn(|| {
			println!("Thread AAA iniciou");			
			thread::sleep(Duration::from_secs(2));
			println!("Thread AAA terminou");
		//	panic!("Thread AAA panicou");
		});

	 	// Cria thread BBB
	 	scope.spawn(|| {
			println!("Thread BBB iniciou");
			thread::sleep(Duration::from_secs(5));
			println!("Thread BBB terminou");
		});

	 	// Cria thread CCC
	 	scope.spawn(|| {
			println!("Thread CCC iniciou");
			thread::sleep(Duration::from_secs(10));
			println!("Thread CCC terminou");
		});

		//_ = handle.join();

	});

	// Todas as threads dentro do escopo já terminaram
	println!("Thread main:  Terminou");
}



