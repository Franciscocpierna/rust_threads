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

O 'join' pode retornar a propriedade (ownership) de um valor ao final da thread criada ?

Sim.
*/


use std::thread;
use std::time::Duration;

fn main() {
	let handle = thread::spawn(|| {
		let mut total = 0;
		for i in 1..10 {
			total += i;
			println!("Thread filha está na contagem {}, total {}",i,total);
			thread::sleep(Duration::from_millis(1));
		}
		format!{"### Thread filha chegou ao total de {} ###", total}
	});

	for i in 1..5 {
		println!("Thread main está na contagem {}",i);
		thread::sleep(Duration::from_millis(1));
    }

	let s = handle.join().unwrap();	// Espera a thread filha terminar

	println!("Thread main recebeu: '{}'", s);

	println!("Thread main: terminou");
}

