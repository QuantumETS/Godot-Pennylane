use godot::prelude::*;
use godot::engine::Node;
use godot::engine::INode;
use spinoza::{
    config::{Config, QSArgs},
    core::{State, CONFIG},
    gates::{apply, Gate},
    utils::{pretty_print_int, to_table},
};
// here are technology that could be integrated/choosen from
//MUST HAVE :
//https://github.com/QuState/spinoza
//https://github.com/delapuente/qasmsim
//BONUS/to look into (in order of how pertinent i think they are):
//https://github.com/HQSquantumsimulations/qoqo_examples
//https://github.com/Q1tBV/q1tsim
//https://github.com/hajifkd/rusq
//https://github.com/MucTepDayH16/qvnt/
//https://qcgpu.github.io/
//https://github.com/28Smiles/qukit <- wasm candidate
//https://github.com/cqs-thu/qns-3
//https://github.com/mtauraso/QuantumSimulator
//https://github.com/beneills/quantum <- meh
//https://github.com/sorin-bolos/moara/blob/master/moara/src/simulator.rs <- not accessible
//Attempts : 
//https://github.com/Renmusxd/RustQIP <- tried it, bad ergonomics

#[derive(GodotClass)]
#[class(base=Node)]
struct QuantumCircuit {
    circuit: Option<State>,
    speed: f64,
    angular_speed: f64,
    base: Base<Node>
}

#[godot_api]
impl INode for QuantumCircuit {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console
        
        Self {
            circuit: None,
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }
    fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be: 
        // rotation += angular_speed * delta
        
        //let radians = (self.angular_speed * delta * self.speed) as f32;
        //self.base_mut().rotate(radians);
        // The 'rotate' method requires a f32, 
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
        //let rotation = self.base().get_rotation();
        //let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        //self.base_mut().translate(velocity * delta as f32);
        
        // or verbose: 
        // let this = self.base_mut();
        // this.set_position(
        //     this.position() + velocity * delta as f32
        // );
    }
}

#[godot_api]
impl QuantumCircuit {
    #[func]
    fn init_circuit(&mut self, nb_qubits: i64, nb_bits: i64) {
       self.circuit = Some(State::new(nb_qubits as usize));
    }
    #[func]
    fn x(&mut self,qubits_nb: i64) {

    }
    fn add_measurement(&mut self, qubits_nb: i64) {
    }
    fn measure(&mut self,){
        
    }
}