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

Threads podem ter nomes ?

Sim, usando o 'Builder', que também pode ser usado para alterar o tamanho da pilha da thread.
*/


use std::thread;
use std::time::Duration;

const N_THREADS: usize = 3;


fn main() {
	let nomes_das_threads = vec!["XXX","YYY","ZZZ"];

	let codigo_thread = || { 
		for i in 1..10 {
			println!("{} está na contagem {}", thread::current().name().unwrap_or("SEM-NOME"),i);
			thread::sleep(Duration::from_millis(1000));
			//panic!("Socorro");
		}
	};

	for nt in 0..N_THREADS {
		let builder = thread::Builder::new().name( nomes_das_threads[nt].to_string() );
		builder.spawn(codigo_thread).unwrap();
	}

    for i in 1..5 {
        println!("Thread main: está na contagem {}",i);
        thread::sleep(Duration::from_millis(1000));
    }

	println!("Thread main: terminou !");

}


