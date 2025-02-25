/*

The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes youâ€™re using Rust 1.76.0 (released 2024-02-08) or later. 
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

Como manter a thread destinatária recebendo mensagens até a destruição do canal ?

Pode-se usar um 'while'.
*/


use std::time::Duration;
use std::sync::mpsc::{self,Sender,Receiver};
use std::thread;


fn thread_que_envia(tx:Sender<String>) {
	tx.send("Mensagem 11111".to_string()).unwrap();
	thread::sleep(Duration::from_secs(4));
	tx.send("Mensagem 22222".to_string()).unwrap();
	thread::sleep(Duration::from_secs(10));
}


fn thread_que_recebe(rx:Receiver<String>) {
	while let Ok(recebida) = rx.recv() {
		println!("Thread_que_recebe: Mensagem recebida: {}", recebida);
	} 
	println!("Thread_que_recebe: Este canal nÃ£o tem mais remetentes ativos!");
}


fn main() {
	// Cria o canal
	let (tx, rx) = mpsc::channel();

	// Cria a thread que envia
	let handle_1 = thread::spawn(move || {
									thread_que_envia(tx);
									} );

	// Cria a thread que recebe
	let handle_2 = thread::spawn(move || {
									thread_que_recebe(rx);
									} );
	
	// Espera ambas terminarem
	_ = handle_1.join();		// Ignoro se a thread em questÃ£o panicou ou nÃ£o
	_ = handle_2.join();

	println!("Thread main: Terminou");
}


