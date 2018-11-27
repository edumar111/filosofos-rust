use std::thread;
use std::time::Duration;

struct Filosofo{
	nombre: String,
}

impl Filosofo{
	fn new(nombre: &str) -> Filosofo{
		Filosofo{
			nombre: nombre.to_string(),
		}
	}
	fn comer(&self){
		println!("{} esta comiendo.", self.nombre);
		//thread::sleep_ms(1000);
		thread::sleep(Duration::from_millis(1000));
		println!("{} ha finalizado de comer", self.nombre);
	}
}
fn main() {
    //let f1 = Filosofo::new("Judith Butler");
    //let f2 = Filosofo::new("Gilles Deleuze");
    //let f3 = Filosofo::new("Karl Max");
    //let f4 = Filosofo::new("Emma Goldman");
    //let f5 = Filosofo::new("Michael Foucault");
    let filosofos = vec![
    	Filosofo::new("Judith Butler"),
        Filosofo::new("Gilles Deleuze"),
        Filosofo::new("Karl Max"),
        Filosofo::new("Emma Goldman"),
        Filosofo::new("Michael Foucault"),
    ];
    
    for f in &filosofos {
    	f.comer();
    }
}
