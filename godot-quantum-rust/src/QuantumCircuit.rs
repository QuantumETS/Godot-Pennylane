use godot::prelude::*;
use godot::engine::Node;
use godot::engine::INode;

use crate::SpinozaSimulator::SpinozaSimulatorStruct;
use std::collections::HashMap;
use qasmsim::statevector::{assert_approx_eq, Complex, StateVector};
use std::f64::consts::FRAC_1_SQRT_2;
use qasmsim;


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

pub trait QuantumSimulator {
    fn init_circuit(&mut self, nb_qubits: i64, nb_bits: i64);
    fn x(&mut self, qubits_nb: i64);
    fn y(&mut self, qubits_nb: i64);
    fn z(&mut self, qubits_nb: i64);
    fn h(&mut self, qubits_nb: i64);
    fn p(&mut self, qubits_nb: i64, value:f64);
    fn rx(&mut self, qubits_nb: i64, value:f64);
    fn ry(&mut self, qubits_nb: i64, value:f64);
    fn rz(&mut self, qubits_nb: i64, value:f64);
    fn identity(&mut self, qubits_nb: i64);
    fn swap(&mut self, qubits_nb_1: i64, qubits_nb_2: i64);
    fn cnot(&mut self, control_qubit_nb: i64, target_qubit_nb: i64);
    fn custom_controlled(&mut self, control_qubit_nb: i64, target_qubit_nb: i64, gatename_x_y_z_rx_ry_rz_h_p:GString, value:f64);
    fn add_measurement(&mut self, qubits_nb: i64);
    fn get_expectation_value(&mut self, measurement_axis_x_y_z:GString);
    fn measure_all(&mut self) -> Array<u8>;
    //default implementation for qasm simulator using qasmsim
    fn run_qasm_str_statevector(&mut self, qasm_string:GString, shots:i64) -> Dictionary
    {
        // statevector() return the StateVector of the circuit in a dictionary containing an array containing dictionaries
        let source =  match qasmsim::run(qasm_string.to_string().as_str(), None) {
            Ok(result) => result.statevector().clone(),
            Err(e) => {godot_print!("result error {:?}", e); StateVector::new(2)}
        };

        let mut godot_dict = Dictionary::new();
        godot_dict.insert("qubit_width", source.qubit_width() as i64);
    
        // Convert complex bases to an array of dictionaries
        let mut bases_array = Array::new();
        for complex in source.as_complex_bases() {
            let mut complex_dict = Dictionary::new();
            complex_dict.insert("re", complex.re);
            complex_dict.insert("im", complex.im);
            bases_array.push(complex_dict);
        }
    
        godot_dict.insert("bases", bases_array);
        godot_dict
    }
    fn run_qasm_str_probabilities(&mut self, qasm_string:GString, shots:i64) -> Array<f64>
    {
        // probabilities() return the probabilities of obtaining each state
        let source =  match qasmsim::run(qasm_string.to_string().as_str(), Some(shots as usize)) {
            Ok(result) => result.probabilities().clone(),
            Err(e) => {
                godot_print!("error in run_qasm_str_probabilities {:?}", e);
                vec![0.0]
            },
        };

        let mut godot_array = Array::new();
        for value in source {
            godot_array.push(value); //reeee
        }
    
        godot_array
    }
    fn run_qasm_str_memory(&mut self, qasm_string:GString, shots:i64) -> Dictionary
    {
        // memory() return a HashMap<String, u64> which has the classical names and the classical outcomes
        let source: HashMap<String, u64> =  match qasmsim::run(qasm_string.to_string().as_str(), Some(shots as usize)) {
            Ok(result) => result.memory().clone(), // cloning reeeeee
            Err(e) => { 
                godot_print!("error in run_qasm_str_memory {:?}", e);
                HashMap::new()
            }
        };

        let mut godot_dict = Dictionary::new();
        for (key, value) in source {
            godot_dict.insert(key, value); //reconstructing manually, reeeeee
        }
    
        godot_dict
    }
    fn run_qasm_str_histogram(&mut self, qasm_string:GString, shots:i64)
    {
        //possible value that can be gotten from the simulator
        // statevector() StateVector,
        // probabilities() Vec<f64>,
        // memory() HashMap<String, u64>,
        // histogram() Option<Histogram>,
        // times() ExecutionTimes,
        // let source =  match qasmsim::run(qasm_string.to_string().as_str(), None) {
        //     Ok(result) => result.histogram(),
        //     Err(e) => godot_print!("result error {:?}", e),
        // };
    }
}


/* enum for the dropdown menu */
#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum Simulator {
    Spinoza,
    Qasmsim,
}

fn default_simulator() -> Box<dyn QuantumSimulator> {
    Box::new(SpinozaSimulatorStruct::new())
}


#[derive(GodotClass)]
#[class(base=Node)]
struct QuantumCircuit {
    quantumSimulator : Box<dyn QuantumSimulator>, // actual simulator "object"
    #[export]
    simulator: Simulator, // enum value selected from the dropdown menu
    base: Base<Node>
}


#[godot_api]
impl INode for QuantumCircuit {
    fn init(base: Base<Node>) -> Self {        
        Self {
            quantumSimulator: default_simulator(), // actual simulator "object", default is spinozasimulator
            simulator: Simulator::Spinoza, // enum value selected from the dropdown menu
            base,
        }
    }
}

#[godot_api]
impl QuantumCircuit {
    #[func]
    fn init_circuit(&mut self, nb_qubits: i64, nb_bits: i64) {
        self.quantumSimulator = match self.simulator{
            Simulator::Spinoza => {Box::new(SpinozaSimulatorStruct::new())}
            Simulator::Qasmsim => {default_simulator()} // not implemented yet, use spinoza as default
        };
        self.quantumSimulator.init_circuit(nb_qubits, nb_bits);
    }

    #[func]
    fn x(&mut self, qubits_nb: i64) {
        self.quantumSimulator.x(qubits_nb);
    }

    #[func]
    fn y(&mut self, qubits_nb: i64) {
        self.quantumSimulator.y(qubits_nb);
    }

    #[func]
    fn z(&mut self, qubits_nb: i64) {
        self.quantumSimulator.z(qubits_nb);
    }

    #[func]
    fn h(&mut self, qubits_nb: i64) {
        self.quantumSimulator.h(qubits_nb);
    }

    #[func]
    fn p(&mut self, qubits_nb: i64, value:f64) { 
        self.quantumSimulator.p(qubits_nb,value);
    }

    #[func]
    fn rx(&mut self, qubits_nb: i64, value:f64) {
        self.quantumSimulator.rx(qubits_nb,value);
    }

    #[func]
    fn ry(&mut self, qubits_nb: i64, value:f64) { 
        self.quantumSimulator.ry(qubits_nb,value);
    }

    #[func]
    fn rz(&mut self, qubits_nb: i64, value:f64) {
        self.quantumSimulator.rz(qubits_nb,value);
    }

    #[func]
    fn identity(&mut self, qubits_nb: i64) {
        self.quantumSimulator.identity(qubits_nb);
    }

    #[func]
    fn swap(&mut self, qubits_nb_1: i64, qubits_nb_2: i64) {
        self.quantumSimulator.swap(qubits_nb_1,qubits_nb_2);
    }

    #[func]
    fn cnot(&mut self, control_qubit_nb: i64, target_qubit_nb: i64) {
        self.quantumSimulator.cnot(control_qubit_nb, target_qubit_nb);
    }

    #[func]
    fn custom_controlled(&mut self, control_qubit_nb: i64, target_qubit_nb: i64, gatename_x_y_z_rx_ry_rz_h_p:GString, value:f64) {
        self.quantumSimulator.custom_controlled(control_qubit_nb, target_qubit_nb, gatename_x_y_z_rx_ry_rz_h_p, value);
    }

    #[func]
    fn add_measurement(&mut self, qubits_nb: i64) {
        self.quantumSimulator.add_measurement(qubits_nb);
    }

    #[func]
    fn get_expectation_value(&mut self, measurement_axis_x_y_z:GString) { 
        self.quantumSimulator.get_expectation_value(measurement_axis_x_y_z);
    }
    #[func]
    fn measure_all(&mut self) -> Array<u8> { //currently, we return a u8 per binary result, we could concatenate the different results into fewer variable/virtual u1 instead.
        self.quantumSimulator.measure_all()
    }
    #[func]
    fn run_qasm_str_probabilities(&mut self, qasm_string:GString, shots:i64) -> Array<f64>
    {
        self.quantumSimulator.run_qasm_str_probabilities(qasm_string, shots)
    }
    #[func]
    fn run_qasm_str_memory(&mut self, qasm_string:GString, shots:i64) -> Dictionary
    {
        self.quantumSimulator.run_qasm_str_memory(qasm_string, shots)
    }
    #[func]
    fn run_qasm_str_statevector(&mut self, qasm_string:GString, shots:i64) -> Dictionary
    {
        self.quantumSimulator.run_qasm_str_statevector(qasm_string, shots)
    }
}
