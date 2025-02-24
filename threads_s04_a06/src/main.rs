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

Como dar parâmetros diferentes para cada uma das várias threads criadas (scoped) ?

Coloca o código das threads em funções.
*/

use std::thread;
use std::time::Duration;


fn thread_tamanho(id: &usize, s: &String) -> usize {
	println!("Thread filha {}:  Recebeu {} com tamanho {}",id,s,s.len());
	thread::sleep(Duration::from_millis(1000));
	s.len()
}


fn main() {

	let dados = vec!["AAAAA".to_string(),
								"BBBBBB".to_string(),
								"CCCCCCC".to_string(),
								"DDDDDDDD".to_string()];
	let ids = vec![0,1,2,3];

	// Cria as threads filhas
	let ret_scope = thread::scope( |scope| {

		let mut handles = Vec::new();

		for i in 0..dados.len() {
			let id = &ids[i];
			let s = &dados[i];
			handles.push( scope.spawn( || {
				thread_tamanho(id,s)	// Valores referenciados por 'id' e 's' 
			} ) );						// precisam viver mais do que a thread
		}

		// Espera todas as threads filhas terminarem
		let mut total = 0;
		for h in handles.into_iter() {
			if let Ok(x) = h.join() {
				total += x;
			}
		}
		total
	});

	println!("Thread main:  Scope retornou {}", ret_scope);

    for i in 1..5 {
        println!("Thread main:  Está na contagem {}",i);
        thread::sleep(Duration::from_millis(1000));
    }

	println!("dados: {:?}", dados);
	println!("Thread main:  Terminou !");

}

