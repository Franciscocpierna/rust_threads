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

O mesmo canal aceita diferentes tipos de dados ?

Não.
*/


// mpsc = Multiple Producer, Single Consumer
use std::sync::mpsc;

fn main() {
	let msg1 = String::from("Minha mensagem é esta.");
	let msg2 = 123.4;

	let (tx, rx) = mpsc::channel();


	// Envio de mensagens com método 'send'
	let _ = tx.send(msg1);
	let _ = tx.send(msg2);

	// Recepção de mensagens com método 'recv'
	if let Ok(recebida) = rx.recv() {
		println!("Thread main: Mensagem recebida: {}", recebida);
	} else {
		println!("Este canal não tem mais remetentes ativos!");
	}

	println!("Thread main: Terminou");
   
}


