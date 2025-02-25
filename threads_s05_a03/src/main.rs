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

Existe 'receive' com bloqueio e timeout ?

Sim.
*/

use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::RecvTimeoutError;

fn main() {
	let msg = String::from("Minha mensagem é esta.");

	let (tx, rx) = mpsc::channel();


	// Envio de mensagens com método 'send'
	// Método 'send' retorna um Result, Ok p/ tem thread ativa no destino, Err p/ não existe thread destinatária.
	// Ok não garante que a mensagem será recebida, apenas que poderá ser recebida (mensagem foi bufferizada).
	let _ = tx.send(msg);

	//println!("{}",msg);	// Valor 'msg' foi movido
	//tx.send(val).unwrap();  // Aborta thread se não existe destinatário ativo


	// Recepção de mensagens com método 'recv_timeout'
	// Não bloqueia para sempre, retorna com mensagem ou no timeout.
	// Método 'recv_timeout' retorna um Result:
	//	Ok p/ mensagem recebida
	//	Err p/ timeout sem mensagem ou não existe remetente ativo.

	match rx.recv_timeout(Duration::from_secs(4)) {
		Ok(recebida) => println!("Thread main: Mensagem recebida: {}", recebida),
		Err(RecvTimeoutError::Timeout) =>	println!("Thread main: Timeout e nenhuma mensagem recebida"),
		Err(RecvTimeoutError::Disconnected) => println!("Thread main: Nenhum remetente ativo"),
	}

	println!("Thread main: recv_timeout com espera de 4 segundos");

	match rx.recv_timeout(Duration::from_secs(4)) {
		Ok(recebida) => println!("Thread main: Mensagem recebida: {}", recebida),
		Err(RecvTimeoutError::Timeout) =>	println!("Thread main: Timeout e nenhuma mensagem recebida"),
		Err(RecvTimeoutError::Disconnected) => println!("Thread main: Nenhum remetente ativo"),
	}

	//let recebida = rx.recv_timeout(Duration::from_secs(4)).unwrap();		// Aborta esta thread se estourar o timeout

	println!("Thread main: Terminou");
   
}




