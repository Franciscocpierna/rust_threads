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

O que acontece quando a thread com o lock do Mutex aborta em pânico ?

O Mutex vai para o estado de 'poisoned'.
*/


use std::time::Duration;
use std::thread;
use std::sync::Mutex;


// Usa lock podendo entrar em pânico
fn thread_1(m_texto: &Mutex<String>) {
	for i in 0..10 {
		thread::sleep(Duration::from_secs(1));					// 1 segundo fora da seção crítica
		let mut x = m_texto.lock().unwrap();
		x.push('O');
		println!("Thread 1 deixou:     '{}'", x);

		if i == 5 {
			panic!("Thread 1 foi abortada com o lock do Mutex!!!");
		}

	}	// Unlock automático
}



// Usa lock sem entrar em pânico
fn thread_2(m_texto: &Mutex<String>) {
	for _ in 0..10 {
		thread::sleep(Duration::from_secs(1));					// 1 segundo fora da seção crítica
		let mut x = m_texto.lock().unwrap();
		x.push('X');
		println!("Thread 2 deixou:     '{}'", x);
	}	// Unlock automático
}


fn main() {
	// Cria Mutex para proteger os dados
	let texto = "".to_string();
	let m_texto = Mutex::new( texto );

	// Cria as threads
	thread::scope( |scope| {
		let handle_1 = scope.spawn( || {thread_1(&m_texto);} );
		let handle_2 = scope.spawn( || {thread_2(&m_texto);} );
		// Espera threads terminarem
		_ = handle_1.join();		// Ignoro se a thread em questão panicou ou não
		_ = handle_2.join();
	} );


	// Coloca o valor final na tela
	//println!("Thread main: texto final '{}'", m_texto.lock().unwrap());

	if let Ok(txt) = m_texto.lock() {
		println!("Thread main: texto final '{}'", txt);
	} else {
		println!("Thread main: mutex está poisoned, thread abortada enquanto detinha o lock");
	}

	println!("Thread main: Terminou");
}



