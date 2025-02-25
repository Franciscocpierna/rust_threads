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

Como funciona a sincronização entre leitores e escritores (RwLock) ?

O RwLock implementa um mecanismo de exclusão mútua que permite múltiplos leitores simultâneos,
mas escritores somente sozinhos.

A política de prioridades depende da implementação subjacente pelo sistema operacional, 
e o Rust não garante que uma política de prioridades em particular será usada.

Existe também 'try_read' e 'try_write'
*/


use rand::Rng;

use std::time::Duration;
use std::thread;
use std::sync::{Arc,RwLock};



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


fn thread_leitora(rw_pessoa: Arc<RwLock<Pessoa>>) {
	println!("Thread_leitora {:?} iniciou", thread::current().id());
	let mut rng = rand::thread_rng();

	// Faz 5 leituras
	for _ in 0..5 {
		thread::sleep(Duration::from_secs(1));		// Fora da seção crítica

		let p = rw_pessoa.read().unwrap();
		println!("Thread_leitora {:?} leu {}", thread::current().id(), p.nome);
		thread::sleep(Duration::from_secs(rng.gen::<u64>() % 4));		// Dentro sa seção crítica
		println!("Thread_leitora {:?} vai sair", thread::current().id());

	}// unlock automático
}


fn thread_escritora(rw_pessoa: Arc<RwLock<Pessoa>>) {
	println!("Thread_escritora {:?} iniciou", thread::current().id());
	let mut rng = rand::thread_rng();

	// Faz 5 escritas
	for _ in 0..5 {
		thread::sleep(Duration::from_secs(1));		// Fora da seção crítica

		let mut p = rw_pessoa.write().unwrap();
		p.nome.push_str("X");
		p.saldo += 1.0;
		println!("Thread_escritora {:?} deixou {}", thread::current().id(), p.nome);
		thread::sleep(Duration::from_secs(rng.gen::<u64>() % 4));		// Dentro da seção crítica
		println!("Thread_escritora {:?} vai sair", thread::current().id());

	}// unlock automático
}



fn main() {

	// Cria o RwLock para proteger 'pessoa'
	let pessoa = Pessoa::new("rômulo".to_string() );
	let rw_pessoa = Arc::new(RwLock::new( pessoa ));

	// Cria threads
	let rw_pessoa_1 = rw_pessoa.clone();
	let rw_pessoa_2 = rw_pessoa.clone();
	let rw_pessoa_3 = rw_pessoa.clone();
	let rw_pessoa_4 = rw_pessoa.clone();
	let rw_pessoa_5 = rw_pessoa.clone();
	let rw_pessoa_6 = rw_pessoa.clone();
	
	let mut handles = Vec::new();

	handles.push( thread::spawn( move || {thread_leitora(rw_pessoa_1);} ) );
	handles.push( thread::spawn( move || {thread_leitora(rw_pessoa_2);} ) );
	handles.push( thread::spawn( move || {thread_leitora(rw_pessoa_3);} ) );
	handles.push( thread::spawn( move || {thread_leitora(rw_pessoa_4);} ) );
	handles.push( thread::spawn( move || {thread_escritora(rw_pessoa_5);} ) );
	handles.push( thread::spawn( move || {thread_escritora(rw_pessoa_6);} ) );

	// Espera threads terminarem
	for h in handles.into_iter() {
		_ = h.join();		// Ignoro se a thread em questão panicou ou não
	}

	println!("Thread main:  Número de cópias existentes depois do join: {}", Arc::strong_count(&rw_pessoa));


	// Obtém o valor interno do Arc se tiver apenas uma referência forte, ou None caso contrário
	let rw = Arc::into_inner(rw_pessoa).unwrap();
	//println!("Thread main:  Número de cópias existentes depois do join: {}", Arc::strong_count(&rw_pessoa));
	//let rw = Arc::try_unwrap(rw_pessoa).unwrap();

	// Destrói o RwLock retornando o conteúdo
	let pessoa = rw.into_inner().unwrap();
	//let _ = rw.read().unwrap();

	// Coloca o valor final na tela
	println!("Thread main: Nome final é '{}'   saldo {}", pessoa.nome, pessoa.saldo);

	println!("Thread main: Terminou");
}
	

	
