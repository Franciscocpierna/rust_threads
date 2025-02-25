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
Podem acontecer Deadlocks em Rust ?

Sim.
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



fn thread_que_deposita(m_pessoa: Arc<Mutex<Pessoa>>, m_depositos: Arc<Mutex<Vec<f64>>>) {
	println!("Thread_que_deposita {:?} iniciou", thread::current().id());
	let valor = 100.0;
	let novo_saldo;

//{
	let mut p = m_pessoa.lock().unwrap();
	p.saldo += valor;
	novo_saldo = p.saldo;
//}
	thread::sleep(Duration::from_millis(1));
	//drop(p);

	let mut d = m_depositos.lock().unwrap();
	d.push(valor);

	thread::sleep(Duration::from_secs(1));
	println!("Thread_que_deposita {:?}: Deixou saldo em {}", thread::current().id(), novo_saldo);
}	// unlock automático



fn thread_que_retira(m_pessoa: Arc<Mutex<Pessoa>>, m_depositos: Arc<Mutex<Vec<f64>>>) {
	println!("Thread_que_retira {:?} iniciou", thread::current().id());
	let valor = 100.0;

	// E se a ordem for invertida ????
	//let mut d = m_depositos.lock().unwrap();
	//d.push(-valor);

	let mut p = m_pessoa.lock().unwrap();
	p.saldo -= valor;

	let mut d = m_depositos.lock().unwrap();
	d.push(-valor);

	thread::sleep(Duration::from_secs(1));
	println!("Thread_que_retira {:?}: Deixou saldo em {}", thread::current().id(), p.saldo);
}	// unlock automático




fn main() {
	// Cria o Mutex para proteger 'pessoa'
	let pessoa = Pessoa::new("rômulo".to_string() );
	let m_pessoa = Arc::new(Mutex::new( pessoa ));

	// Cria o Mutex para proteger 'depositos'
	let depositos: Vec<f64> = Vec::new();
	let m_depositos = Arc::new(Mutex::new( depositos ));


	// Cria threads que depositam
	let m_pessoa_1 = m_pessoa.clone();
	let m_pessoa_2 = m_pessoa.clone();
	let m_pessoa_3 = m_pessoa.clone();
	let m_pessoa_4 = m_pessoa.clone();

	let m_depositos_1 = m_depositos.clone();
	let m_depositos_2 = m_depositos.clone();
	let m_depositos_3 = m_depositos.clone();
	let m_depositos_4 = m_depositos.clone();

	let handle_1 = thread::spawn( move || {
													thread_que_deposita(m_pessoa_1,m_depositos_1);} );
	let handle_2 = thread::spawn( move || {
													thread_que_deposita(m_pessoa_2,m_depositos_2);} );
	let handle_3 = thread::spawn( move || {
													thread_que_retira(m_pessoa_3,m_depositos_3);} );
	let handle_4 = thread::spawn( move || {
													thread_que_retira(m_pessoa_4,m_depositos_4);} );
	

	// Espera todos terminarem
	_ = handle_1.join();		// Ignoro se a thread em questão panicou ou não
	_ = handle_2.join();
	_ = handle_3.join();
	_ = handle_4.join();
	println!("Thread main:  Número de cópias existentes depois do join: {}", Arc::strong_count(&m_pessoa));

	// Coloca o saldo final na tela
	let p = m_pessoa.lock().unwrap();
	println!("Thread main: Saldo final de '{}' é '{}'", p.nome, p.saldo);

	println!("Thread main: Terminou");
}



