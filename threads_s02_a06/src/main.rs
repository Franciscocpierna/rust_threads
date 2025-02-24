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

Como repetir o mesmo código em várias threads ?

Pode usar uma variável com a closure que contém o código das threads.
*/

use std::thread;
use std::time::Duration;

const N_THREADS: usize = 3;

fn main() {

	let codigo_thread = || { 
		for i in 1..10 {
            println!("{:?} está na contagem {}",thread::current().id(),i);
			thread::sleep(Duration::from_millis(1000));
		}
	};

	for _nt in 0..N_THREADS {
		thread::spawn(codigo_thread);
	}

    for i in 1..5 {
        println!("Thread main: está na contagem {}",i);
        thread::sleep(Duration::from_millis(1000));
    }

	println!("Thread main: terminou !");

}


