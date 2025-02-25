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

Qual a capacidade de bufferização (armazenagem) de um canal ?

Infinita.
*/

use std::time::Duration;
use std::sync::mpsc::{self,Sender,Receiver};
use std::thread;


fn thread_que_envia(tx:Sender<String>) {
	tx.send("AAAAAAAAAA".to_string()).unwrap();
	tx.send("BBBBBBBBBB".to_string()).unwrap();
	tx.send("CCCCCCCCCC".to_string()).unwrap();
	tx.send("DDDDDDDDDD".to_string()).unwrap();
	tx.send("EEEEEEEEEE".to_string()).unwrap();
	tx.send("FFFFFFFFFF".to_string()).unwrap();
	println!("Thread_que_envia: Terminou");
}

fn thread_que_recebe(rx:Receiver<String>) {
	//'rx' como um iterator, quando o canal é fechado a iteração termina
	for recebido in rx {
		println!("Thread_que_recebe: Recebeu '{}'", recebido);
		thread::sleep(Duration::from_secs(2));
	}
	println!("Thread_que_recebe: Terminou");
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

