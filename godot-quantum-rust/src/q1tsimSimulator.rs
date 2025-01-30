use godot::log::godot_print;
use crate::QuantumCircuit::QuantumSimulator;
use godot::prelude::*;
use q1tsim::{circuit, gates::{self}};


pub struct q1tsimSimulatorStruct {
    circuit: Option<circuit::Circuit>,
    circuit_size: i64,
}

impl q1tsimSimulatorStruct {
    pub fn new() -> Self {
        godot_print!("Running q1tsim Simulator");
        Self { 
            circuit:None,
            circuit_size:0, 
        }
    }
}

impl QuantumSimulator for q1tsimSimulatorStruct {
    fn init_circuit(&mut self, nb_qubits: i64, nb_bits: i64) {
        let mut nb_measurement_bits = nb_bits;
        if nb_bits < nb_qubits {nb_measurement_bits = nb_qubits;}
        self.circuit = Some(circuit::Circuit::new(nb_qubits as usize, nb_measurement_bits as usize));
        self.circuit_size = nb_qubits;
    }

    fn x(&mut self, qubits_nb: i64) {
        if let Some(ref mut circuit) = self.circuit {
            circuit.x(qubits_nb as usize).unwrap();
        } else {
            godot_print!("Circuit is not initialized!");
        }
    }

    fn y(&mut self, qubits_nb: i64) {
        if let Some(ref mut circuit) = self.circuit {
            circuit.y(qubits_nb as usize).unwrap();
        } else {
            godot_print!("Circuit is not initialized!");
        }
    }

    fn z(&mut self, qubits_nb: i64) {
        if let Some(ref mut circuit) = self.circuit {
            circuit.z(qubits_nb as usize).unwrap();
        } else {
            godot_print!("Circuit is not initialized!");
        }
    }

    fn h(&mut self, qubits_nb: i64) {
        if let Some(ref mut circuit) = self.circuit {
            circuit.h(qubits_nb as usize).unwrap();
        } else {
            godot_print!("Circuit is not initialized!");
        }
    }

    fn p(&mut self, qubits_nb: i64, value:f64) { //phase shift
        if let Some(ref mut circuit) = self.circuit {
            circuit.rz(value, qubits_nb as usize).unwrap();
        } else {
            godot_print!("Circuit is not initialized!");
        }
    }

    fn s(&mut self, qubits_nb: i64) {
        if let Some(ref mut circuit) = self.circuit {
            circuit.s(qubits_nb as usize).unwrap();
        } else {
            godot_print!("Circuit is not initialized!");
        }
    }

    fn rx(&mut self, qubits_nb: i64, value:f64) { 
        if let Some(ref mut circuit) = self.circuit {
            circuit.rx(value, qubits_nb as usize).unwrap();
        } else {
            godot_print!("Circuit is not initialized!");
        }
    }

    fn ry(&mut self, qubits_nb: i64, value:f64) { 
        if let Some(ref mut circuit) = self.circuit {
            circuit.ry(value, qubits_nb as usize).unwrap();
        } else {
            godot_print!("Circuit is not initialized!");
        }
    }

    fn rz(&mut self, qubits_nb: i64, value:f64) { 
        if let Some(ref mut circuit) = self.circuit {
            circuit.rz(value, qubits_nb as usize).unwrap();
        } else {
            godot_print!("Circuit is not initialized!");
        }
    }

    fn identity(&mut self, _qubits_nb: i64) {
    }

    fn swap(&mut self, qubits_nb_1: i64, qubits_nb_2: i64) {
        if let Some(ref mut circuit) = self.circuit {
            circuit.add_gate(gates::Swap::new(), &[qubits_nb_1 as usize, qubits_nb_2 as usize]).unwrap();
        } else {
            godot_print!("State is not initialized!");
        }
    }

    fn cnot(&mut self, control_qubit_nb: i64, target_qubit_nb: i64) {
        if let Some(ref mut circuit) = self.circuit {
            circuit.cx(control_qubit_nb as usize,target_qubit_nb as usize).unwrap();
        } else {
            godot_print!("State is not initialized!");
        }
    }

    fn custom_controlled(&mut self, control_qubit_nb: i64, target_qubit_nb: i64, gatename_x_y_z_rx_ry_rz_h_p:GString, value:f64) {
        if let Some(ref mut circuit) = self.circuit {
            let _result = match gatename_x_y_z_rx_ry_rz_h_p.to_string().as_str() {
                "x" => Ok(circuit.add_gate(gates::CX::new(),&[control_qubit_nb as usize, target_qubit_nb as usize]).unwrap()),
                "y" => Ok(circuit.add_gate(gates::CY::new(),&[control_qubit_nb as usize, target_qubit_nb as usize]).unwrap()),
                "z" => Ok(circuit.add_gate(gates::CZ::new(),&[control_qubit_nb as usize, target_qubit_nb as usize]).unwrap()),
                "rx" => Ok(circuit.add_gate(gates::CRX::new(value),&[control_qubit_nb as usize, target_qubit_nb as usize]).unwrap()),
                "ry" => Ok(circuit.add_gate(gates::CRY::new(value),&[control_qubit_nb as usize, target_qubit_nb as usize]).unwrap()),
                "rz" => Ok(circuit.add_gate(gates::CRZ::new(value),&[control_qubit_nb as usize, target_qubit_nb as usize]).unwrap()),
                "h" => Ok(circuit.add_gate(gates::CH::new(),&[control_qubit_nb as usize, target_qubit_nb as usize]).unwrap()),
                "p" => Ok(circuit.add_gate(gates::CRZ::new(value),&[control_qubit_nb as usize, target_qubit_nb as usize]).unwrap()),
                _ => {godot_print!("Custom controlled operation gate error, please choose among the following : x y z rx ry rz h p"); Err(())},
            };
        } else { godot_print!("circuit  is not initialized!");}
    }

    fn add_measurement(&mut self, _qubits_nb: i64) {
        if let Some(ref mut circuit) = self.circuit {
            godot_print!("add measurement is not implemented for this simulator!");
            //apply(Gate::M, circuit, qubits_nb as usize);
        } else {
            godot_print!("Circuit is not initialized!");
        }
    }

    fn get_expectation_value(&mut self, measurement_axis_x_y_z:GString) -> Array<f64> { 
        // let now = std::time::Instant::now();
        // let elapsed = now.elapsed().as_micros();
        // let targets = (0..self.circuit_size as usize).collect::<Vec<usize>>();
        // if let Some(ref circuit) = self.circuit {
        //     let exp_vals = xyz_expectation_value(measurement_axis_x_y_z.to_string().chars().next().unwrap(), circuit, &targets);
        //     godot_print!("expectation values: {:?}", exp_vals);
        // }
        godot_print!("expectation values not implemented or available on this simulator");
        Array::<f64>::new()
    }

    fn measure_all(&mut self, shots:i64) -> Array<GString> { //currently, we return a u8 per binary result, we could concatenate the different results into fewer variable/virtual u1 instead.
        let now = std::time::Instant::now();
        let mut arr: Array<GString> = Array::new();
        if let Some(ref mut circuit) = self.circuit {
            let indices: Vec<usize> = (0..(self.circuit_size-1) as usize).collect();
            circuit.measure_all(&indices).unwrap();
            circuit.execute(shots as usize).unwrap();
            let hist = circuit.histogram_string().unwrap();
            for (bits, count) in hist
            {
                for _ in 0..count{
                    arr.push(GString::from(bits.clone()));
                }
                godot_print!("{}: {}", bits, count);
            }
        }
        let elapsed = now.elapsed().as_micros();
        godot_print!(
            "Caculated in {}s",
            elapsed
        );
        arr
    }
}