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

Como mandar diferentes tipos de dados pelo mesmo canal ?

Pode-se usar um 'enum'.
*/

use std::sync::mpsc::{self,Sender,Receiver};
use std::thread;

enum MinhaMensagem {
	String(String),
	Float(f64),
}


fn thread_que_envia(tx:Sender<MinhaMensagem>) {
	let msg1 = "Mensagem secreta".to_string();
	let msg2 = 123.4;
	tx.send(MinhaMensagem::String(msg1)).unwrap();
	tx.send(MinhaMensagem::Float(msg2)).unwrap();
}


fn thread_que_recebe(rx:Receiver<MinhaMensagem>) {
	loop {
		if let Ok(recebida) = rx.recv() {
			match recebida {
				MinhaMensagem::Float(x) => println!("Thread_que_recebe: Recebeu float: '{}'", x),
				MinhaMensagem::String(x) => println!("Thread_que_recebe: Recebeu String: '{}'", x),
			}
		} else {
			println!("Thread_que_recebe: Este canal não tem mais remetentes ativos!");
			break;
		}
	}
}	


fn main() {
	// Cria o canal
	let (tx, rx) = mpsc::channel();

	// Cria a thread que envia
	let handle_1 = thread::spawn(move || { thread_que_envia(tx);} );

	// Cria a thread que recebe
	let handle_2 = thread::spawn(move || { thread_que_recebe(rx);} );

	// Espera ambas terminarem
	_ = handle_1.join();		// Ignoro se a thread em questão panicou ou não
	_ = handle_2.join();

	println!("Thread main: Terminou");
}

