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
Como apenas emprestar valores imutáveis para as threads criadas ?

Usando 'scoped threads'.
*/

use std::thread;
use std::time::Duration;


fn main() {
	let msg = "%%%%%".to_string();
	let vv = vec![1, 2, 3];
	//let mut vv = vec![1, 2, 3];										// Pode ser mutável ?

	let ret_scope = thread::scope( |scope| {

		let handle_x = scope.spawn(|| {
			for i in 1..5 {
				println!("Thread filha XXX: Aqui está {vv:?} {msg}");
				//vv[0] += 1;											// Pode ser mutável ?
				println!("Thread filha XXX está na contagem {}",i);
				thread::sleep(Duration::from_millis(1));
			}
			println!("Thread filha XXX:  Terminou");
		});

		let handle_y = scope.spawn(|| {
			for i in 1..5 {
				println!("Thread filha YYY: Aqui está {vv:?} {msg}");
				//vv[0] += 1;											// Pode ser mutável ?
				println!("Thread filha YYY está na contagem {}",i);
				thread::sleep(Duration::from_millis(1));
			}
			//panic!("!!!Thread filha YYY: Não sei o que fazer");	
			println!("Thread filha YYY:  Terminou");
		});

		// Pode chamar 'scope.spawn' várias vezes para criar várias threads
		// Função 'scope' só termina depois que todas as threads aqui dentro terminarem
		// Opcionalmente pode usar 'join' explícito caso queira tratar o retorno das threads criadas
		let mut n_ok = 0;

		if handle_x.join().is_ok() {
			n_ok += 1;
		};
		if handle_y.join().is_ok() {
			n_ok += 1;
		};

		n_ok
	});

	// Neste ponto a função 'scope' já terminou e suas threads também
	println!("Thread main:  Função scope retornou número de Ok= {}", ret_scope);

	// E agora como fica isto ?
	println!("Thread main:  Posso acessar vv e msg ?  {:?} {}", vv, msg);

	println!("Thread main:  Terminou");

}


