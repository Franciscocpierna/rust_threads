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

Como fazer quando várias estruturas de dados independentes são compartilhadas pelas threads ?

Usar um Mutex diferente para cada estrutura de dados independente.
*/


use std::time::Duration;
use std::thread;
use std::sync::Mutex;


#[derive(Debug)]
struct Pessoa {
	nome: String,
	saldo: f64,
}
impl Pessoa {
	fn new(nome:String) -> Pessoa {
		Pessoa {
			nome,
			saldo: 0.0,
		}
	}
}


// Faz dez depósitos de 1 real, espera 1s antes de cada depósito
fn thread_que_deposita(pessoa: &Mutex<Pessoa>, saldos: &Mutex<Vec<f64>>) {
	for _ in 0..10 {
		thread::sleep(Duration::from_secs(1));
		let mut p = pessoa.lock().unwrap();
		p.saldo += 1.00;

		if p.saldo > 15.0 {
			let mut s = saldos.lock().unwrap();
			s.push(p.saldo);
		} // Unlock automático p/saldos

		println!("Thread {:?}: Deixou saldo de {} em {}", thread::current().id(), p.nome, p.saldo);
	}	// Unlock automático p/pessoa
}



fn main() {
	// Cria o Mutex para proteger Pessoa
	let pessoa = Mutex::new( Pessoa::new("rômulo".to_string()) );
	let saldos: Mutex<Vec<f64>> = Mutex::new( Vec::new() );

	// Estruturas de dados
	println!("pessoa:  {:?}", pessoa);
	println!("depositos:  {:?}", saldos);


	// Cria threads que depositam

	thread::scope( |scope| {
		let handle_1 = scope.spawn( || {thread_que_deposita(&pessoa,&saldos);} );
		let handle_2 = scope.spawn( || {thread_que_deposita(&pessoa,&saldos);} );
		let handle_3 = scope.spawn( || {thread_que_deposita(&pessoa,&saldos);} );
		// Espera threads terminarem
		_ = handle_1.join();		// Ignoro se a thread em questão panicou ou não
		_ = handle_2.join();
		_ = handle_3.join();
	} );

	// Coloca o saldo final na tela
	println!("Thread main: Saldo final é {}", pessoa.lock().unwrap().saldo);

	// Estruturas de dados
	println!("pessoa:  {:?}", pessoa);
	println!("depositos:  {:?}", saldos);

	println!("Thread main: Terminou");
}



