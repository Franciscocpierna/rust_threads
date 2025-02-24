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

Como fazer uma thread esperar até que outra thread termine ?

Usando o método 'join'.
*/

use std::thread;
use std::time::Duration;

fn main() {
	let handle = thread::spawn(|| {
		for i in 1..10 {
			println!("Thread filha está na contagem {}",i);
			thread::sleep(Duration::from_millis(1));
		}
	});

	handle.join().unwrap();	// Espera a thread filha terminar (serializa execução)

	for i in 1..5 {
		println!("Thread main está na contagem {}",i);
		thread::sleep(Duration::from_millis(1));
    }

	//handle.join().unwrap();	// Espera a thread filha terminar

	// Observação
	//handle.join().unwrap();	// 'join' consome 'handle', ou seja, 'handle' é movido para 'join'
	
	println!("Thread main terminou !");
}



