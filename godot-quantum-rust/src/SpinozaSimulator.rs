use godot::log::godot_print;
use crate::QuantumCircuit::QuantumSimulator;
use godot::prelude::*;
use spinoza::{
    core::{qubit_expectation_value, xyz_expectation_value, State}, gates::{apply, c_apply, Gate}, measurement::measure_qubit, utils::to_table
};
// interesting but not implemented feature from this simulator : qcbm, value_encoding, custom multi controlled
pub struct SpinozaSimulatorStruct {
    circuit: Option<State>,
    circuit_size: i64,
}

impl SpinozaSimulatorStruct {
    pub fn new() -> Self {
        Self { 
            circuit:None,
            circuit_size:0, 
        }
    }
}

impl QuantumSimulator for SpinozaSimulatorStruct {
    fn init_circuit(&mut self, nb_qubits: i64, _nb_bits: i64) {
        self.circuit = Some(State::new(nb_qubits as usize));
        self.circuit_size = nb_qubits;
    }

    fn x(&mut self, qubits_nb: i64) {
        if let Some(ref mut circuit) = self.circuit {
            apply(Gate::X, circuit, qubits_nb as usize);
        } else {
            godot_print!("State is not initialized!");
        }
    }

    fn y(&mut self, qubits_nb: i64) {
        if let Some(ref mut circuit) = self.circuit {
            apply(Gate::Y, circuit, qubits_nb as usize);
        } else {
            godot_print!("State is not initialized!");
        }
    }

    fn z(&mut self, qubits_nb: i64) {
        if let Some(ref mut circuit) = self.circuit {
            apply(Gate::Z, circuit, qubits_nb as usize);
        } else {
            godot_print!("State is not initialized!");
        }
    }

    fn h(&mut self, qubits_nb: i64) {
        if let Some(ref mut circuit) = self.circuit {
            apply(Gate::H, circuit, qubits_nb as usize);
        } else {
            godot_print!("State is not initialized!");
        }
    }

    fn p(&mut self, qubits_nb: i64, value:f64) { //phase shift
        if let Some(ref mut circuit) = self.circuit {
            apply(Gate::P(value), circuit, qubits_nb as usize);
        } else {
            godot_print!("State is not initialized!");
        }
    }

    fn rx(&mut self, qubits_nb: i64, value:f64) {
        if let Some(ref mut circuit) = self.circuit {
            apply(Gate::RX(value), circuit, qubits_nb as usize);
        } else {
            godot_print!("State is not initialized!");
        }
    }

    fn ry(&mut self, qubits_nb: i64, value:f64) { 
        if let Some(ref mut circuit) = self.circuit {
            apply(Gate::RY(value), circuit, qubits_nb as usize);
        } else {
            godot_print!("State is not initialized!");
        }
    }

    fn rz(&mut self, qubits_nb: i64, value:f64) {
        if let Some(ref mut circuit) = self.circuit {
            apply(Gate::RZ(value), circuit, qubits_nb as usize);
        } else {
            godot_print!("State is not initialized!");
        }
    }

    fn s(&mut self, qubits_nb: i64) {
        if let Some(ref mut circuit) = self.circuit {
            apply(Gate::RZ(std::f64::consts::PI/2.0), circuit, qubits_nb as usize);
        } else {
            godot_print!("State is not initialized!");
        }
    }

    fn identity(&mut self, _qubits_nb: i64) {
    }

    fn swap(&mut self, qubits_nb_1: i64, qubits_nb_2: i64) {
        // if let Some(ref mut circuit) = self.circuit {
        //     apply(Gate::SWAP(qubits_nb_1,qubits_nb_2), circuit);
        // } else {
        //     godot_print!("State is not initialized!");
        // }
        godot_print!("swap not implemented for this simulator")
    }

    fn cnot(&mut self, control_qubit_nb: i64, target_qubit_nb: i64) {
        if let Some(ref mut circuit) = self.circuit {
            c_apply(Gate::X, circuit, control_qubit_nb as usize, target_qubit_nb as usize);
        } else {
            godot_print!("State is not initialized!");
        }
    }

    fn custom_controlled(&mut self, control_qubit_nb: i64, target_qubit_nb: i64, gatename_x_y_z_rx_ry_rz_h_p:&GString, value:f64) {
        let result = match gatename_x_y_z_rx_ry_rz_h_p.to_string().as_str() {
            "x" => Ok(Gate::X),
            "y" => Ok(Gate::Y),
            "z" => Ok(Gate::Z),
            "rx" => Ok(Gate::RX(value)),
            "ry" => Ok(Gate::RY(value)),
            "rz" => Ok(Gate::RZ(value)),
            "h" => Ok(Gate::H),
            "p" => Ok(Gate::P(value)),
            _ => Err(()),
        }; // if we measure a custom controlled gate with something like an rx gate with a value at 0.22, for some reason, it crashes in measurement.rs from spinoza, works elsewhere 
        if let Ok(gate) = result {
            if let Some(ref mut circuit) = self.circuit {
                c_apply(gate, circuit, control_qubit_nb as usize, target_qubit_nb as usize);
            } else { godot_print!("State is not initialized!");}
        }
        else { godot_print!("Custom controlled operation gate error"); }
    }

    fn add_measurement(&mut self, qubits_nb: i64) {
        if let Some(ref mut circuit) = self.circuit {
            apply(Gate::M, circuit, qubits_nb as usize);
        } else {
            godot_print!("State is not initialized!");
        }
    }

    fn get_expectation_value(&mut self, measurement_axis_x_y_z:GString) -> Array<f64> { 
        let now = std::time::Instant::now();
        let targets = (0..self.circuit_size as usize).collect::<Vec<usize>>();
        if let Some(ref circuit) = self.circuit {
            let exp_vals = xyz_expectation_value(measurement_axis_x_y_z.to_string().chars().next().unwrap(), circuit, &targets); // their function doesn't properly compute the expectation value
            //let exp_vals: Vec<_> = (0..(self.circuit_size as usize)).map(|t| qubit_expectation_value(&circuit, t)).collect();
            let mut godot_array = Array::new();
            for value in exp_vals {
                godot_array.push(value); //reeee
            }
            let elapsed = now.elapsed().as_micros();
            godot_print!("time to calculate expectation value : .{:?}s", elapsed.to_string());
            godot_array
        } else { 
            Array::<f64>::new()
        }
    }

    fn measure_all(&mut self, _shots:i64) -> Array<GString> {
        godot_print!("Running Spinoza Simulator");
        let now = std::time::Instant::now();
        let mut arr: Array<GString> = Array::new();
        let mut to_submit : String = String::new();
        if let Some(ref mut circuit) = self.circuit {
            for t in 0..self.circuit_size {
                let temp = measure_qubit(circuit, t as usize, true, None).to_string();
                to_submit = format!("{to_submit}{temp}");
            }
        }
        arr.push(GString::from(to_submit));
        let elapsed = now.elapsed().as_micros();
        godot_print!(
            "circuit result :  {} \nCaculated in {}s",
            to_table(&self.circuit.clone().unwrap()),
            elapsed
        );
        arr
    }

    fn get_statevector(&mut self) -> Dictionary
    {
        let circuit = self.circuit.clone().unwrap();
        let mut dict = Dictionary::new();

        let mut reals_array = Array::new();
        let mut imags_array = Array::new();
        
        for &real in &circuit.reals {
            reals_array.push(Variant::from(real)); // Convert f64 to Variant
        }
        for &imag in &circuit.imags {
            imags_array.push(Variant::from(imag)); // Convert f64 to Variant
        }

        dict.insert("reals", reals_array);
        dict.insert("imags", imags_array); 
        dict   
    }
}