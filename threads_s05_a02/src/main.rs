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

Existe 'receive' sem bloqueio ?

Sim.
*/


use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;

fn main() {
	let msg = String::from("Minha mensagem é esta.");

	let (tx, rx) = mpsc::channel();


	// Envio de mensagens com método 'send'
	// Método 'send' retorna um Result, Ok p/ tem thread ativa no destino, Err p/ não existe thread destinatária.
	// Ok não garante que a mensagem será recebida, apenas que poderá ser recebida (mensagem foi bufferizada).
	let _ = tx.send(msg);

	//println!("{}",msg);	// Valor 'msg' foi movido
	//tx.send(val).unwrap();  // Aborta thread se não existe destinatário ativo


	// Recepção de mensagens com método 'try_recv'
	// Nunca bloqueia, retorna imediatamente
	// Método 'try_recv' retorna um Result:
	//	Ok p/ mensagem recebida
	//	Err p/ não existe mensagem ou não existe remetente ativo
	match rx.try_recv() {
		Ok(recebida) => println!("Thread main: Mensagem recebida: {}", recebida),
		Err(TryRecvError::Empty) =>	println!("Thread main: Nenhuma mensagem recebida"),
		Err(TryRecvError::Disconnected) => println!("Thread main: Nenhum remetente ativo"),
	}

	match rx.try_recv() {
		Ok(recebida) => println!("Thread main: Mensagem recebida: {}", recebida),
		Err(TryRecvError::Empty) =>	println!("Thread main: Nenhuma mensagem recebida"),
		Err(TryRecvError::Disconnected) => println!("Thread main: Nenhum remetente ativo"),
	}

	//let recebida = rx.try_recv().unwrap();  // Aborta esta thread se não tiver mensagem disponível

	println!("Thread main: Terminou");
   
}



