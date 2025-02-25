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

O que é um Canal e como são criados ?

O Canal é uma caixa-postal onde mensagens podem ser depositadas.
*/


// mpsc = Multiple Producer, Single Consumer
use std::sync::mpsc;

fn main() {
	let msg = String::from("Minha mensagem é esta.");

	let (tx, rx) = mpsc::channel();


	// Envio de mensagens com método 'send'
	// Método 'send' retorna um Result, Ok p/ tem thread ativa no destino, Err p/ não existe thread destinatária.
	// Ok não garante que a mensagem será recebida, apenas que poderá ser recebida (mensagem foi bufferizada).
	let _ = tx.send(msg);

	//println!("{}",msg);		// Valor 'msg' foi movido
	//tx.send(msg).unwrap();	// Aborta thread se não existe destinatário ativo


	// Recepção de mensagens com método 'recv'
	// Bloqueia até receber uma mensagem ou não existirem mais remetentes ativos
	// Método 'recv' retorna um Result, Ok p/ mensagem recebida, Err p/ não existe mensagem nem remetente ativo
	if let Ok(recebida) = rx.recv() {
		println!("Thread main: Mensagem recebida: {}", recebida);
	} else {
		println!("Este canal não tem mais remetentes ativos!");
	}

	//let recebida = rx.recv().unwrap();  	// Aborta programa inteiro se não existe remetente ativo

	println!("Thread main: Terminou");
   
}


