use godot::engine::file_access::ModeFlags;
use godot::engine::file_dialog::FileMode;
use godot::engine::file_dialog::Access;
use godot::engine::FileAccess;
use godot::engine::FileDialog;
use godot::obj::NewAlloc;
use godot::prelude::*;
use godot::engine::Node;
use godot::engine::INode;

use crate::q1tsimSimulator::q1tsimSimulatorStruct;
use crate::QasmExporter::QasmExporterStruct;
use crate::SpinozaSimulator::SpinozaSimulatorStruct;
use std::collections::HashMap;
use qasmsim::statevector::StateVector;
use qasmsim::Histogram;
use qasmsim;


// here are technology that could be integrated/choosen from
//MUST HAVE :
//https://github.com/delapuente/qasmsim
//BONUS/to look into (in order of how pertinent i think they are):
//https://github.com/HQSquantumsimulations/qoqo_examples
//https://github.com/hajifkd/rusq
//https://github.com/MucTepDayH16/qvnt/
//https://qcgpu.github.io/
//https://github.com/28Smiles/qukit <- wasm candidate
//https://github.com/cqs-thu/qns-3
//https://github.com/mtauraso/QuantumSimulator
//https://github.com/beneills/quantum <- meh
//https://github.com/sorin-bolos/moara/blob/master/moara/src/simulator.rs <- not accessible
//Done but has bugs :
//https://github.com/QuState/spinoza
//https://github.com/Q1tBV/q1tsim
//Attempts : 
//https://github.com/Renmusxd/RustQIP <- tried it, bad ergonomics

pub trait QuantumSimulator {
    fn init_circuit(&mut self, nb_qubits: i64, nb_bits: i64);
    fn x(&mut self, qubits_nb: i64);
    fn y(&mut self, qubits_nb: i64);
    fn z(&mut self, qubits_nb: i64);
    fn h(&mut self, qubits_nb: i64);
    fn p(&mut self, qubits_nb: i64, value:f64);
    fn s(&mut self, qubits_nb: i64);
    fn rx(&mut self, qubits_nb: i64, value:f64);
    fn ry(&mut self, qubits_nb: i64, value:f64);
    fn rz(&mut self, qubits_nb: i64, value:f64);
    fn identity(&mut self, qubits_nb: i64);
    fn swap(&mut self, qubits_nb_1: i64, qubits_nb_2: i64);
    fn cnot(&mut self, control_qubit_nb: i64, target_qubit_nb: i64);
    fn custom_controlled(&mut self, control_qubit_nb: i64, target_qubit_nb: i64, gatename_x_y_z_rx_ry_rz_h_p:&GString, value:f64);
    fn add_measurement(&mut self, qubits_nb: i64);
    fn get_expectation_value(&mut self, measurement_axis_x_y_z:GString) -> Array<f64>
    {
        godot_print!("expectation values not available on this simulator");
        Array::<f64>::new()
    }
    fn get_statevector(&mut self) -> Dictionary
    {
        godot_print!("no statevector available on this simulator");
        Dictionary::new()
    }
    fn measure_all(&mut self, shots:i64) -> Array<GString>;
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
    fn run_qasm_str_histogram(&mut self, qasm_string:GString, shots:i64) -> Dictionary
    {
        let default_histogram: Histogram = HashMap::new();

        // histogram() return the number of time a particular value was measured in a histogram
        let source =  match qasmsim::run(qasm_string.to_string().as_str(), Some(shots as usize)) {
            Ok(result) => match result.histogram().clone() {
                Some(result) => result,
                None => {godot_print!("None in run_qasm_str_histogram "); default_histogram}
            },
            Err(e) => {
                godot_print!("error in run_qasm_str_histogram {:?}", e); 
                default_histogram
            }
        };

        // Convert Histogram to Godot Dictionary
        let mut godot_dict = Dictionary::new();
        for (key, value_vec) in source {
            let mut godot_array = Array::new();
            for (measured_value, count) in value_vec {
                let mut pair = Dictionary::new();
                pair.insert("measured_value", measured_value);
                pair.insert("count", count as i64);
                godot_array.push(pair);
            }
            godot_dict.insert(key, godot_array);
        }

        godot_dict
    }

}


/* enum for the dropdown menu */
#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum Simulator {
    q1tsim,
    Spinoza,
}

fn default_simulator() -> Box<dyn QuantumSimulator> {
    Box::new(q1tsimSimulatorStruct::new())
}


#[derive(GodotClass)]
#[class(base=Node)]
struct QuantumCircuit {
    quantum_simulator : Box<dyn QuantumSimulator>, // actual simulator "object"
    #[export]
    simulator: Simulator, // enum value selected from the dropdown menu
    base: Base<Node>,
    qasm_exporter : QasmExporterStruct
}


#[godot_api]
impl INode for QuantumCircuit {
    fn init(base: Base<Node>) -> Self {        
        Self {
            quantum_simulator: default_simulator(), // actual simulator "object", default is spinozasimulator
            simulator: Simulator::Spinoza, // enum value selected from the dropdown menu
            qasm_exporter: QasmExporterStruct::default(),
            base,
        }
    }
}

#[godot_api]
impl QuantumCircuit {
    #[func]
    /// Initialise a quantum circuit on the quantum simulator. This function should be called first before doing anything else (unless using the qasm simulator).
    /// nb_qubits specify the number of Qubits in the circuits that must be initialized. nb_bits specify the number of classical bits used for storing measurement.
    fn init_circuit(&mut self, nb_qubits: i64, nb_bits: i64) {
        self.quantum_simulator = match self.simulator {
            Simulator::q1tsim => {Box::new(q1tsimSimulatorStruct::new())},
            Simulator::Spinoza => {Box::new(SpinozaSimulatorStruct::new())},
        };
        self.quantum_simulator.init_circuit(nb_qubits, nb_bits);
        self.qasm_exporter.init_circuit(nb_qubits,nb_qubits); // same number of measurement bit as qubits, might change if an application is found where it would be constraining
    }

    #[func]
    /// Applies the X gate to the specified qubit.
    fn x(&mut self, qubits_nb: i64) {
        self.quantum_simulator.x(qubits_nb);
        self.qasm_exporter.add_single_qubit_gate("x", qubits_nb, None);
    }

    #[func]
    /// Applies the Y gate to the specified qubit.
    fn y(&mut self, qubits_nb: i64) {
        self.quantum_simulator.y(qubits_nb);
        self.qasm_exporter.add_single_qubit_gate("y", qubits_nb, None);
    }

    #[func]
    /// Applies the Z gate to the specified qubit.
    fn z(&mut self, qubits_nb: i64) {
        self.quantum_simulator.z(qubits_nb);
        self.qasm_exporter.add_single_qubit_gate("z", qubits_nb, None);
    }

    #[func]
    /// Applies the H gate to the specified qubit.
    fn h(&mut self, qubits_nb: i64) {
        self.quantum_simulator.h(qubits_nb);
        self.qasm_exporter.add_single_qubit_gate("h", qubits_nb, None);
    }

    #[func]
    /// Applies the P (Phase) gate to the specified qubit with a given value (in radians).
    fn p(&mut self, qubits_nb: i64, value:f64) { 
        self.quantum_simulator.p(qubits_nb,value);
        self.qasm_exporter.add_single_qubit_gate("p", qubits_nb, Some(value));
    }
    #[func]
    fn s(&mut self, qubits_nb: i64) {
        self.quantum_simulator.s(qubits_nb);
        self.qasm_exporter.add_single_qubit_gate("s", qubits_nb, None);
    }
    #[func]
    /// Applies the RX (Rotation around X-axis) gate to the specified qubit with a given value (in radians).
    fn rx(&mut self, qubits_nb: i64, value:f64) {
        self.quantum_simulator.rx(qubits_nb,value);
        self.qasm_exporter.add_single_qubit_gate("rx", qubits_nb, Some(value));
    }

    #[func]
    /// Applies the RY (Rotation around Y-axis) gate to the specified qubit with a given value (in radians).
    fn ry(&mut self, qubits_nb: i64, value:f64) { 
        self.quantum_simulator.ry(qubits_nb,value);
        self.qasm_exporter.add_single_qubit_gate("ry", qubits_nb, Some(value));
    }

    #[func]
    /// Applies the RZ (Rotation around Z-axis) gate to the specified qubit with a given value (in radians).
    fn rz(&mut self, qubits_nb: i64, value:f64) {
        self.quantum_simulator.rz(qubits_nb,value);
        self.qasm_exporter.add_single_qubit_gate("rz", qubits_nb, Some(value));
    }

    #[func]
    /// Applies the Identity gate to the specified qubit. Does nothing to the qubit.
    fn identity(&mut self, qubits_nb: i64) {
        self.quantum_simulator.identity(qubits_nb);
        self.qasm_exporter.add_single_qubit_gate("id", qubits_nb, None);
    }

    #[func]
    /// Applies the SWAP gate to exchange the states of two qubits.
    fn swap(&mut self, qubits_nb_1: i64, qubits_nb_2: i64) {
        self.quantum_simulator.swap(qubits_nb_1,qubits_nb_2);
        self.qasm_exporter.add_controlled_qubit_gate("swap", qubits_nb_1, qubits_nb_2, None);
    }

    #[func]
    /// Applies the Controlled-NOT (CNOT) gate to two qubits.
    fn cnot(&mut self, control_qubit_nb: i64, target_qubit_nb: i64) {
        self.quantum_simulator.cnot(control_qubit_nb, target_qubit_nb);
        self.qasm_exporter.add_controlled_qubit_gate("cx", control_qubit_nb, target_qubit_nb, None);
    }

    #[func]
    /// Applies a custom controlled gate to the specified qubits.
    /// The controlled gate is determined by its name and an optional parameter value.
    fn custom_controlled(&mut self, control_qubit_nb: i64, target_qubit_nb: i64, gatename_x_y_z_rx_ry_rz_h_p:GString, value:f64) {
        self.quantum_simulator.custom_controlled(control_qubit_nb, target_qubit_nb, &gatename_x_y_z_rx_ry_rz_h_p, value);
        let has_value = match gatename_x_y_z_rx_ry_rz_h_p.to_string().as_str() {
            "x" => None,
            "y" => None,
            "z" => None,
            "h" => None,
            "rx" => Some(value),
            "ry" => Some(value),
            "rz" => Some(value),
            "p" => Some(value),
            _ => None,
        }; 
        self.qasm_exporter.add_controlled_qubit_gate(&format!("c{}", gatename_x_y_z_rx_ry_rz_h_p), control_qubit_nb, target_qubit_nb, has_value);
    }

    #[func]
    /// Adds a measurement operation to the specified qubit.
    fn add_measurement(&mut self, qubits_nb: i64) {
        self.quantum_simulator.add_measurement(qubits_nb);
        self.qasm_exporter.add_measurement(qubits_nb);
    }

    #[func]
    /// Retrieves the expectation value of a measurement along a specific axis.
    fn get_expectation_value(&mut self, measurement_axis_x_y_z:GString) -> Array<f64> { 
        self.quantum_simulator.get_expectation_value(measurement_axis_x_y_z)
    }

    #[func]
    /// Measures all qubits and returns the results. Each result corresponds to a binary outcome for each qubit.
    fn measure_all(&mut self, shots:i64) -> Array<GString> { //currently, we return a u8 per binary result, we could concatenate the different results into fewer variable/virtual u1 instead.
        self.quantum_simulator.measure_all(shots)
    }
    
    #[func]
    /// Get the statevector of the circuit if available
    fn get_statevector(&mut self) -> Dictionary
    {
        self.quantum_simulator.get_statevector()
    }

    #[func]
    /// Runs a QASM string to compute the probabilities of outcomes for a given number of shots.
    fn run_qasm_str_probabilities(&mut self, qasm_string:GString, shots:i64) -> Array<f64>
    {
        self.quantum_simulator.run_qasm_str_probabilities(qasm_string, shots)
    }
    #[func]
    /// Runs a QASM string to compute the memory of measurement results for a given number of shots.
    fn run_qasm_str_memory(&mut self, qasm_string:GString, shots:i64) -> Dictionary
    {
        self.quantum_simulator.run_qasm_str_memory(qasm_string, shots)
    }
    #[func]
    /// Runs a QASM string to retrieve the state vector.
    fn run_qasm_str_statevector(&mut self, qasm_string:GString, shots:i64) -> Dictionary
    {
        self.quantum_simulator.run_qasm_str_statevector(qasm_string, shots)
    }
    #[func]
    /// Runs a QASM string to compute the measurement histogram for a given number of shots.
    fn run_qasm_str_histogram(&mut self, qasm_string:GString, shots:i64) -> Dictionary
    {
        self.quantum_simulator.run_qasm_str_histogram(qasm_string, shots)
    }

    #[func]
    /// Return the openqasm program in the format of a string
    fn export_to_openqasm_string(&mut self) -> GString
    {
        GString::from(self.qasm_exporter.export_qasm())
    }

    #[func]
    fn export_to_openqasm_file_w_dialog(&mut self) {
        let mut file_dialog = FileDialog::new_alloc();
        file_dialog.set_file_mode(FileMode::SAVE_FILE);
        file_dialog.set_access(Access::FILESYSTEM);
        file_dialog.add_filter(GString::from("*.qasm"));
        file_dialog.set_use_native_dialog(true); 
        
        // Connect "file_selected" signal to call the Rust function
        file_dialog.connect("file_selected".into(), self.base().callable("export_to_openqasm_file"));
        file_dialog.call_deferred(StringName::from("popup_centered"), &[]); // need to be deferred to ensure that the object is in the scene tree before being called
        let fd_clone = file_dialog.clone(); // clone a reference to the object, not the object itself.

        file_dialog.connect("confirmed".into(), fd_clone.callable("queue_free")); // destroy the dialog after use, so that we don't endup creating multiple copies of it
        file_dialog.connect("canceled".into(), fd_clone.callable("queue_free"));
        file_dialog.show();
        self.base_mut().add_child(file_dialog.upcast());
        
    }
    /// Use godot file system to export and save a .qasm file
    #[func]
    fn export_to_openqasm_file(&mut self, path: GString)
    {
        let exported_qasm_string = GString::from(self.qasm_exporter.export_qasm());
        let mut received_path = path;

        // If the path doesn't end with ".qasm", append it.
        if !received_path.to_string().ends_with(".qasm") {
            let new_path = format!("{}.qasm", received_path.to_string());
            received_path = GString::from(new_path);
        }

        
        let file = FileAccess::open(received_path.clone(), ModeFlags::WRITE);
        
        if let Some(mut file) = file {
            file.store_string(exported_qasm_string);
            godot_print!("File written successfully to: {}", received_path);
        } else {
            godot_print!("Failed to open file: {}", received_path);
        }
    }
}
