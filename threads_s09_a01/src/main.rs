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

Como funciona uma barreira (Barrier) ?

A barreira garante que um grupo de threads, ao chegarem na barreira, 
irão esperar até que todas do grupo cheguem, antes de continuarem a execução.
*/


//use rand::Rng;

use std::time::Duration;
use std::thread;
use std::sync::{Mutex,Barrier};

// Número de tarefas calculadoras
const N_CALCULADORAS: usize = 8;

// Número de elementos no vetor de dados
const N_ELEMENTOS: usize = 10000;

// Usado para passar parâmetros na criação de threads
#[derive(Debug)]
struct ThParam {
	id: usize,			// Identificação da thread
	inicio: usize,		// Posição inicial a ser pesquisada
	fim: usize,			// Posição final a ser pesquisada
}


// Localiza o maior valor em um bloco de dados e atualiza resultado geral
fn thread_calculadora( param: &ThParam, dados: &[f64], m_resultados: &Mutex<Vec<f64>>,
							b_primeira:&Barrier, b_segunda:&Barrier) {
	println!("Thread {:?}:  {:?}", thread::current().id(), param);

	// Calcula o máximo de seu bloco de dados
	let mut maximo_local = f64::NEG_INFINITY;
	for x in dados.iter() {
		if maximo_local < *x {
			maximo_local = *x;
		}
	}
	println!("Thread {} tem máximo local {}", param.id, maximo_local);

	// Insere no vector compartilhado de resultados
	{
		let mut res = m_resultados.lock().unwrap();
		res[param.id] = maximo_local;
	}	// unlock automático

	// Espera pela primeira barreira, todos fizeram o calculo parcial
	let b_resultado = b_primeira.wait();

	// Aqui só trabalha se for a thread especial
	if b_resultado.is_leader() {
		println!("Thread {:?} é a Líder e vai descansar 1s...", thread::current().id());
		thread::sleep(Duration::from_secs(1));		// Faz todo mundo esperar

		let mut maximo_global = f64::NEG_INFINITY;
		let mut res = m_resultados.lock().unwrap();
		for x in res.iter() {
			if maximo_global < *x {
				maximo_global = *x;
			}
		}
		println!("Thread {} descobriu máximo global {}", param.id, maximo_global);
		for x in res.iter_mut() {
			*x -= maximo_global;
		}
	}	// unlock automático

	// Espera pela segunda barreira, calculo geral feito pela líder
	b_segunda.wait();

	// Cada thread mostra o seu resultado individual
	{
		let res = m_resultados.lock().unwrap();
		println!("Thread {} faltou {} para o máximo", param.id, res[param.id]);
	}	// unlock automático	

}



fn main() {
	// Inicializa gerador de números aleatórios
	//let mut rng = rand::thread_rng();

	// Cria o vetor de dados
	let mut dados  = [0.0;N_ELEMENTOS];
	for i in 0..N_ELEMENTOS {
		dados[i] = i as f64;					// Melhor para depuração
		//dados[i] = rng.gen();
	}

	// Prepara os parâmetros para as threads
	let mut th_params = Vec::new();
	for t in 0..N_CALCULADORAS {
		th_params.push(
			ThParam{
				id: t,										// Identificação da thread
				inicio: t * (N_ELEMENTOS/N_CALCULADORAS),	// Posição inicial a ser pesquisada
				fim: (t+1) * (N_ELEMENTOS/N_CALCULADORAS),	// Posição final a ser pesquisada
			}
		);
	}

	// Cria vetor de resultados
		let mut th_resultados = Vec::new();
		for _ in 0..N_CALCULADORAS {
			th_resultados.push(f64::NEG_INFINITY);		// Maior valor encontrado
		}

	// Cria o Mutex para proteger o vetor de resultados
	let m_resultados = Mutex::new( th_resultados );
	println!("Thread Main: {:?}", m_resultados);

	// Cria as duas barreiras
	let fim_primeira_fase = Barrier::new(N_CALCULADORAS);
	let fim_segunda_fase = Barrier::new(N_CALCULADORAS);

	// Executa as threads calculadoras
	thread::scope( |scope| {
		let mut handles = Vec::new();

		for p in th_params.iter() {
			handles.push( scope.spawn( || {	thread_calculadora(
												p,
												&dados[p.inicio..p.fim],
												&m_resultados,
												&fim_primeira_fase,
												&fim_segunda_fase);
											} ) );
		}

		// Espera threads terminarem, para não abortar a thread main
		for h in handles.into_iter() {
			_ = h.join();		// Ignoro se a thread em questão panicou ou não
		}
	});


	// Coloca o resultado final na tela
	println!("Thread main: Resultado final é {:?}", m_resultados.lock().unwrap());

	println!("Thread main: Terminou");
}


