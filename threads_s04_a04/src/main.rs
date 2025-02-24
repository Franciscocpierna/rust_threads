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

Como dar parâmetros diferentes para cada uma das várias threads criadas (move)?

Coloca o código das threads em funções.
*/

use std::thread::{self,JoinHandle};
use std::time::Duration;


fn cria_thread_contadora(limite: i32) -> JoinHandle<()> {
	thread::spawn( move|| {
		for i in 1..limite {
			println!("Thread filha com limite:{}: está na contagem {}",limite,i);
			thread::sleep(Duration::from_millis(1000));
		}
	})	
}


fn main() {

	// Cria as threads filhas
	let mut handles = Vec::new();

	for limite in 7..10 {
		handles.push( cria_thread_contadora(limite) );
	}

    for i in 1..5 {
        println!("Thread main: está na contagem {}",i);
        thread::sleep(Duration::from_millis(1000));
    }

	// Espera todas as threads filhas terminarem
	for h in handles.into_iter() {
		h.join().unwrap();		// Precisa ownership, por isto 'into_iter'
	}	

	println!("Thread main: terminou !");

}



