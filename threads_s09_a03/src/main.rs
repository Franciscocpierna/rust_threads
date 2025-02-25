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

Como funciona uma Variável Condição (Condvar) ?

Condvar permite bloquear uma thread enquanto ela espera por um evento.
São normalmente usadas junto com um Mutex, o qual protege o teste da condição de bloqueio.
*/

use rand::Rng;

use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex, Condvar};



fn thread_deposita( m_saldo: Arc<Mutex<f64>>, c_saldo: Arc<Condvar>) {
	println!("Thread_deposita {:?} iniciou", thread::current().id());
	let mut rng = rand::thread_rng();

	// Faz 5 depósitos
	for _ in 0..5 {
		thread::sleep(Duration::from_secs(1 + rng.gen::<u64>() % 4));		// Fora da seção crítica
		let valor_a_depositar = (rng.gen::<u64>() % 100) as f64;

		let mut s = m_saldo.lock().unwrap();
		*s += valor_a_depositar;
		println!("Thread_deposita {:?} deixou {}", thread::current().id(), *s);
		// Notifica alguma thread que possa estar esperando pelo dinheiro
		//c_saldo.notify_one();
		c_saldo.notify_all();
	}// unlock automático

}


fn thread_retira( m_saldo: Arc<Mutex<f64>>, c_saldo: Arc<Condvar>) {
	println!("Thread_retira {:?} iniciou", thread::current().id());
	let mut rng = rand::thread_rng();

	// Faz 5 retiradas
	for _ in 0..5 {
		thread::sleep(Duration::from_secs(1 + rng.gen::<u64>() % 4));		// Fora da seção crítica
		let valor_a_retirar = (rng.gen::<u64>() % 100) as f64;

		let mut s = m_saldo.lock().unwrap();
		while  *s < valor_a_retirar {
			println!("Thread_retira {:?} espera para retirar {}", thread::current().id(), valor_a_retirar);
			s = c_saldo.wait(s).unwrap();
		}
		*s -= valor_a_retirar;
		println!("Thread_retira {:?} deixou {}", thread::current().id(), *s);
	}// unlock automático
	
}



fn main() {

	// Cria os dados a serem protegidos
	let saldo = 0.0;

	// Cria o par Mutex e Condvar
	let m_saldo = Arc::new(Mutex::new(saldo));
	let c_saldo = Arc::new(Condvar::new());

	// Cria threads
	let mut handles = Vec::new();

	let m_saldo_1 = m_saldo.clone();
	let c_saldo_1 = c_saldo.clone();
	handles.push( thread::spawn( move || {thread_deposita( m_saldo_1, c_saldo_1);} ) );

	let m_saldo_1 = m_saldo.clone();
	let c_saldo_1 = c_saldo.clone();
	handles.push( thread::spawn( move || {thread_deposita( m_saldo_1, c_saldo_1);} ) );

	let m_saldo_1 = m_saldo.clone();
	let c_saldo_1 = c_saldo.clone();
	handles.push( thread::spawn( move || {thread_retira( m_saldo_1, c_saldo_1);} ) );

	let m_saldo_1 = m_saldo.clone();
	let c_saldo_1 = c_saldo.clone();
	handles.push( thread::spawn( move || {thread_retira( m_saldo_1, c_saldo_1);} ) );

	// Monitora evolução do saldo
	loop {
		thread::sleep(Duration::from_secs(1));
		// Verifica se não existem outras threads	
		if Arc::strong_count(&m_saldo) == 1 {
			break;
		} else {
			let s = m_saldo.lock().unwrap();
			println!("Thread main: Saldo atual é {}", s);
		}
	}

	// Espera threads terminarem, redundante mas seguro
	for h in handles.into_iter() {
		_ = h.join();		// Ignoro se a thread em questão panicou ou não
	}

	// Coloca o valor final na tela
	let s = m_saldo.lock().unwrap();
	println!("Thread main: Saldo final é {}", s);

	println!("Thread main: Terminou");
}


	
	
