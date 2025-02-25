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

Posso ter várias threads recebendo mensagens pelo mesmo canal ?

Não. O nome já diz tudo, mpsc significa 'Multi-Producer, Single-Consumer FIFO queue'
*/


use std::sync::mpsc::{self,Sender,Receiver};
use std::thread;
use std::time::Duration;


fn thread_que_envia(tx:Sender<String>) {
	let valores = vec![ String::from("AAAAA"),String::from("BBBBB"),
									String::from("CCCCC"),String::from("DDDDD")];
	for v in valores.into_iter() {		// Precisa do ownership
		let _ = tx.send(v);
		thread::sleep(Duration::from_secs(1));
	}
	println!("Thread_que_envia: Terminou");
}


fn thread_que_recebe_1(rx:Receiver<String>) {
	for recebido in rx {			// rx como um iterator, termina quando o canal é fechado
		println!("Thread_que_recebe_1: Recebeu '{}'", recebido);
	}
	println!("Thread_que_recebe_1: Canal foi fechado");
	println!("Thread_que_recebe_1: Terminou");
}	


fn thread_que_recebe_2(rx:Receiver<String>) {
	for recebido in rx {			// rx como um iterator, termina quando o canal é fechado
		println!("Thread_que_recebe_2: Recebeu '{}'", recebido);
	}
	println!("Thread_que_recebe_2: Canal foi fechado");
	println!("Thread_que_recebe_2: Terminou");
}	

fn main() {
	// Cria o canal
	let (tx, rx) = mpsc::channel();

	// Cria as threads que recebem
	let rx1 = rx.clone();
	let handle_1 = thread::spawn(move || { thread_que_recebe_1(rx1);} );	// Não pode clonar e é movido
	let handle_2 = thread::spawn(move || { thread_que_recebe_2(rx);} );

	// Cria a thread que envia
	let handle_3 = thread::spawn(move || { thread_que_envia(tx);} );
	
	// Espera ambas terminarem
	_ = handle_1.join();		// Ignoro se a thread em questão panicou ou não
	_ = handle_2.join();
	_ = handle_3.join();

	println!("Thread main: Terminou");
}


