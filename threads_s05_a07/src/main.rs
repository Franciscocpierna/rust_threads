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

Posso ter várias threads enviando mensagens pelo mesmo canal ?

Sim.
*/

use std::sync::mpsc::{self,Sender,Receiver};
use std::thread;
use std::time::Duration;


fn thread_que_envia_1(tx:Sender<String>) {
	let valores = vec![ String::from("AAAAA"),String::from("BBBBB"),
									String::from("CCCCC"),String::from("DDDDD")];
	for v in valores.into_iter() {		// Precisa do ownership
		let _ = tx.send(v);
		thread::sleep(Duration::from_secs(1));
	}
	println!("Thread_que_envia_1: Terminou");
}


fn thread_que_envia_2(tx:Sender<String>) {
	let valores = vec![ String::from("WWWWW"),String::from("XXXXX"),
									String::from("YYYYY"),String::from("ZZZZZ")];
	for v in valores.into_iter() {		// Precisa do ownership
		let _ = tx.send(v);
		thread::sleep(Duration::from_secs(1));
	}
	println!("Thread_que_envia_2: Terminou");
}


fn thread_que_recebe(rx:Receiver<String>) {
	for recebido in rx {			// rx como um iterator, termina quando o canal é fechado
		println!("Thread_que_recebe: Recebeu '{}'", recebido);
	}
	println!("Thread_que_recebe: Canal foi fechado");
	println!("Thread_que_recebe: Terminou");
}	


fn main() {
	// Cria o canal
	let (tx, rx) = mpsc::channel();

	// Cria a thread que recebe
	let handle_1 = thread::spawn(move || { thread_que_recebe(rx);} );

	// Cria as threads que enviam
	let tx1 = tx.clone();
	//let tx2 = tx.clone();	// Deixaria um transmissor ativo

	let handle_2 = thread::spawn(move || { thread_que_envia_1(tx1);} );
	let handle_3 = thread::spawn(move || { thread_que_envia_2(tx);} );
	
	// Espera todas as threads filhas terminarem
	_ = handle_1.join();		// Ignoro se a thread em questão panicou ou não
	_ = handle_2.join();
	_ = handle_3.join();

	println!("Thread main: Terminou");
}


