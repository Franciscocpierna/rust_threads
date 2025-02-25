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

É possível programar no estilo dos Monitores ?

Sim.
*/

use rand::Rng;

use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex, Condvar};


/************************************
*									*
*	Código do Monitor				*
*									*
************************************/
#[derive(Clone)]
struct MonitorSaldo {
	m_saldo: Arc<Mutex<f64>>,
	c_saldo: Arc<Condvar>,
}

impl MonitorSaldo {
	fn new(saldo_inicial: f64) -> MonitorSaldo {
		MonitorSaldo {
			m_saldo: Arc::new(Mutex::new(saldo_inicial)),
			c_saldo: Arc::new(Condvar::new()),
		}
	}

	fn deposita( &self, valor: f64) {
		let mut s = self.m_saldo.lock().unwrap();
		*s += valor;
		println!("Thread_deposita {:?} deixou {}", thread::current().id(), *s);
		// Notifica alguma thread que possa estar esperando pelo dinheiro		
		//c_saldo.notify_one();
		self.c_saldo.notify_all();
	}// unlock automático

	fn retira( &self, valor: f64) {
		let mut s = self.m_saldo.lock().unwrap();
		while  *s < valor {
			println!("Thread_retira {:?} espera para retirar {}", thread::current().id(), valor);
			s = self.c_saldo.wait(s).unwrap();
		}
		*s -= valor;
		println!("Thread_retira {:?} deixou {}", thread::current().id(), *s);
	}// unlock automático

	fn le_saldo(&self) -> f64 {
		let s = self.m_saldo.lock().unwrap();
		*s
	}// unlock automático

	fn contagem(&self) -> usize {
		Arc::strong_count(&self.m_saldo)
	}// unlock automático

}



/************************************
*									*
*	Código das Threads				*
*									*
************************************/


fn thread_deposita( monitor_saldo: MonitorSaldo) {
	println!("Thread_deposita {:?} iniciou", thread::current().id());
	let mut rng = rand::thread_rng();

	// Faz 5 depósitos
	for _ in 0..5 {
		// Sleep fora da seção crítica
		thread::sleep(Duration::from_secs(1 + rng.gen::<u64>() % 4));
		let valor_a_depositar = (rng.gen::<u64>() % 100) as f64;
		monitor_saldo.deposita(valor_a_depositar);
	}
}


fn thread_retira( monitor_saldo: MonitorSaldo) {
	println!("Thread_retira {:?} iniciou", thread::current().id());
	let mut rng = rand::thread_rng();

	// Faz 5 retiradas
	for _ in 0..5 {
		// Sleep fora da seção crítica
		thread::sleep(Duration::from_secs(1 + rng.gen::<u64>() % 4));
		let valor_a_retirar = (rng.gen::<u64>() % 100) as f64;
		monitor_saldo.retira(valor_a_retirar);
	}
}



fn main() {

	// Cria o monitor
	let monitor_saldo = MonitorSaldo::new(0.0);

	// Cria threads
	let mut handles = Vec::new();

	let monitor_saldo_1 = monitor_saldo.clone();
	handles.push( thread::spawn( move || {thread_deposita( monitor_saldo_1);} ) );

	let monitor_saldo_2 = monitor_saldo.clone();
	handles.push( thread::spawn( move || {thread_deposita( monitor_saldo_2);} ) );

	let monitor_saldo_3 = monitor_saldo.clone();
	handles.push( thread::spawn( move || {thread_retira( monitor_saldo_3);} ) );

	let monitor_saldo_4 = monitor_saldo.clone();
	handles.push( thread::spawn( move || {thread_retira( monitor_saldo_4);} ) );

	// Monitora evolução do saldo
	loop {
		thread::sleep(Duration::from_secs(1));
		// Verifica se não existem outras threads	
		if monitor_saldo.contagem() == 1 {
			break;
		} else {
			println!("Thread main: Saldo atual é {}", monitor_saldo.le_saldo());
		}
	}

	// Espera threads terminarem, redundante mas seguro
	for h in handles.into_iter() {
		_ = h.join();		// Ignoro se a thread em questão panicou ou não
	}

	// Coloca o valor final na tela
	println!("Thread main: Saldo final é {}", monitor_saldo.le_saldo());

	println!("Thread main: Terminou");
}
	
