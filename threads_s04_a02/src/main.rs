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

Como passar a propriedade (ownership) de uma variável para a thread filha ?

Usando 'move', pois isto garante que a thread filha tem propriedade 
sobre os valores capturados da thread main.
No caso de valores que implementam o trait COPY, eles são copiados para a thread filha.
*/

use std::thread::{self,JoinHandle};


fn cria_thread_filha() -> JoinHandle<()> {
	let vv1 = vec![1, 2, 3];
	let vv2 = 0;		// E se for um tipo primitivo ?
	
    let handle = thread::spawn(move || {
		println!("Thread filha executa");
		println!("Aqui está vv1: {:?}", vv1 );	// Tem a propriedade (ownership) de 'vv1'
		println!("Aqui está vv2: {:?}", vv2 );	// Tem a propriedade (ownership) de 'vv2'
    });

	// E agora como fica isto ?
	//println!("Thread main:  Posso acessar vv1 e vv2 ?  {:?} {}", vv1, vv2);

	handle
}



fn main() {
	let handle = cria_thread_filha();
	handle.join().unwrap();
	println!("Thread main terminou");
}

