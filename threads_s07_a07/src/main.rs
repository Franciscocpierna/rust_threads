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

É possível executar um 'lock' sem ficar bloqueado ?

Sim, usando 'try_lock()'.
*/

use std::time::Duration;
use std::thread;
use std::sync::Mutex;


// Faz incrementos lentamente
fn thread_lenta(m_texto: &Mutex<String>) {
	for _ in 0..10 {
		thread::sleep(Duration::from_secs(1));					// 1 segundo fora da seção crítica
		let mut x = m_texto.lock().unwrap();
		x.push('O');
		thread::sleep(Duration::from_secs(1));					// 1 segundo dentro da seção crítica
		println!("Thread lenta deixou:     '{}'", x);
	}	// Unlock automático
}



// Usa try_lock
fn thread_com_try_lock(m_texto: &Mutex<String>) {
	for _ in 0..10 {
		loop {
			if let Ok(mut x) = m_texto.try_lock() {
				x.push('X');
				println!("Thread try_lock deixou:  '{}'", x);
				// Unlock automático
				break;
			} else {
				println!("Thread try_lock esperando ...");
				thread::sleep(Duration::from_millis(200));
			}
		}
		thread::sleep(Duration::from_secs(1));		// Fora da seção crítica
	}
}


fn main() {
	// Cria Mutex para proteger os dados
	let texto = "".to_string();
	let m_texto = Mutex::new( texto );

	// Cria as threads
	thread::scope( |scope| {
		let handle_1 = scope.spawn( || {thread_lenta(&m_texto);} );
		let handle_2 = scope.spawn( || {thread_com_try_lock(&m_texto);} );
		// Espera threads terminarem
		_ = handle_1.join();		// Ignoro se a thread em questão panicou ou não
		_ = handle_2.join();
	} );

	// Coloca o valor final na tela
	println!("Thread main: texto final '{}'", m_texto.lock().unwrap());

	println!("Thread main: Terminou");
}



