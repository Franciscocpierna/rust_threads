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

Como fica a questão do ownership quando Mutex é usado ?

Depende do tipo de dado.
*/

use std::time::Duration;
use std::thread;
use std::sync::Mutex;


fn thread_1(m_contagem: &Mutex<f64>, m_texto1: &Mutex<String>, m_texto2: &Mutex<&mut String>) {
	for _ in 0..10 {
		thread::sleep(Duration::from_secs(1));		// 1 segundo fora da seção crítica

		let mut c = m_contagem.lock().unwrap();
		//c += 1.00;		// deref automático não existe para f64
		*c += 1.00;

		let mut t1 = m_texto1.lock().unwrap();
		t1.push('X');
		(*t1).push('x');

		let mut t2 = m_texto2.lock().unwrap();
		t2.push('Y');
		(*t2).push('y');

		println!("Thread 1 deixou contagem {}", c);
	}	// Unlock automático
}



fn thread_2(m_contagem: &Mutex<f64>) {
	for _ in 0..10 {
		thread::sleep(Duration::from_secs(1));					// 1 segundo fora da seção crítica
		let mut c = m_contagem.lock().unwrap();
		*c += 1.00;
		println!("Thread 2 deixou contagem {}", c);
	}	// Unlock automático
}


fn main() {
	// Cria Mutex para proteger os dados
	let contagem = 0.0;
	let m_contagem = Mutex::new( contagem );

	let texto1 = "11111".to_string();
	let m_texto1 = Mutex::new(texto1);

	let mut texto2 = "22222".to_string();
	let m_texto2 = Mutex::new(&mut texto2);


	// Cria as threads
	thread::scope( |scope| {

		let handle_1 = scope.spawn( || {
												thread_1(&m_contagem,&m_texto1,&m_texto2);} );
		let handle_2 = scope.spawn( || {
												thread_2(&m_contagem);} );
		// Espera threads terminarem
		_ = handle_1.join();		// Ignoro se a thread em questão panicou ou não
		_ = handle_2.join();
	} );

	// Coloca os valores finais na tela

	// f64 tem semântica COPY, 'contagem' foi copiado p/Mutex	
	println!("Thread main: contagem final é {}", m_contagem.lock().unwrap());
	println!("Thread main: contagem final é {}", contagem);

	// String tem semântica MOVE, 'texto1' foi movido p/Mutex
	println!("Thread main: texto1 final é {}", m_texto1.lock().unwrap());
	//println!("Thread main: texto1 final é {}", texto1);

	// 'texto2' foi apenas emprestado para o Mutex, mas mutável impede outros empréstimos 
	println!("Thread main: texto2 final é {}", m_texto2.lock().unwrap());
	println!("Thread main: texto2 final é {}", texto2);
	//println!("Thread main: texto2 final é {}", m_texto2.lock().unwrap());

	println!("Thread main: Terminou");
}



