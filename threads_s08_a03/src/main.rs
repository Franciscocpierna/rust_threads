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
Como compartilhar o mesmo Mutex entre threads quaisquer ?

Utiliza Arc - Atomically Reference Counted
	'Reference' -> É uma referência para um valor
	'Counted' -> O número de referências existentes é contado
					A cada clone() o contador é incrementado
					A cada drop (destruição do clone) o contador é decrementado
					Quando o contador chega a zero a memória é liberada 
	'Atomically' -> Várias threads podem clonar/dropar sem problema

Coloca o Mutex dentro de um ARC
Mutex garante acesso mutuamente exclusivo aos dados
Arc permite controlar o número de referências para o Mutex, somente destruindo o Mutex na contagem zero
*/


use std::time::Duration;
use std::thread;
use std::sync::{Mutex,Arc};


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


// Faz dez depósitos de 1 real, espera 1 segundo antes de cada depósito
fn thread_que_deposita(m_pessoa: Arc<Mutex<Pessoa>>) {
	for _ in 0..10 {
		thread::sleep(Duration::from_secs(1));
		let mut p = m_pessoa.lock().unwrap();
		p.saldo += 1.00;
		println!("Thread {:?}: Deixou saldo em {}", thread::current().id(), p.saldo);

		//thread::sleep(Duration::from_secs(1));		// Força todas esperarem por conta do Mutex
	}	// Unlock automático
}



fn main() {
	// Cria o Mutex para proteger Pessoa
	let pessoa =  Pessoa::new("rômulo".to_string() );
	let m_pessoa = Arc::new(Mutex::new( pessoa ));

	// Cria threads que depositam
	let m_pessoa_1 = m_pessoa.clone();
	let m_pessoa_2 = m_pessoa.clone();
	let m_pessoa_3 = m_pessoa.clone();
	println!("Thread main:  Número de cópias existentes antes do join: {}", Arc::strong_count(&m_pessoa));

	//let xx = || {thread_que_deposita(m_pessoa_1);};

	let handle_1 = thread::spawn( move || {thread_que_deposita(m_pessoa_1);} );
	let handle_2 = thread::spawn( move || {thread_que_deposita(m_pessoa_2);} );
	let handle_3 = thread::spawn( move || {thread_que_deposita(m_pessoa_3);} );

	//let ppp = m_pessoa_3.clone();


	// Espera todos terminarem
	_ = handle_1.join();		// Ignoro se a thread em questão panicou ou não
	_ = handle_2.join();
	_ = handle_3.join();
	println!("Thread main:  Número de cópias existentes depois do join: {}", Arc::strong_count(&m_pessoa));

	// Coloca o saldo final na tela
	let p = m_pessoa.lock().unwrap();
	println!("Thread main: Saldo final de '{}' é '{}'", p.nome, p.saldo);

	println!("Thread main: Terminou");
}



