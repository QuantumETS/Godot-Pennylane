use std::path::Path;

use godot::log::godot_print;
use openqasm::{GenericError, Parser, SourceCache};

#[derive(Default)]
pub struct QasmExporterStruct {
    qasm_code: Vec<String>,
}

impl QasmExporterStruct {
    pub fn init_circuit(&mut self, nb_qubits:i64, nb_bits:i64){
        self.qasm_code.clear();
        self.qasm_code.push("OPENQASM 2.0;".to_string());
        self.qasm_code.push("include \"qelib1.inc;\"".to_string());
        self.qasm_code.push(format!("qreg q[{}];", nb_qubits));
        self.qasm_code.push(format!("creg c[{}];", nb_bits));
    }
    pub fn add_single_qubit_gate(&mut self, gate: &str, qubits_nb: i64, value: Option<f64>) {
        let qasm_instruction = match value {
            Some(v) => format!("{}({}) q[{}];", gate, v, qubits_nb.to_string()),
            None => format!("{} q[{}];", gate, qubits_nb.to_string()),
        };
        self.qasm_code.push(qasm_instruction);
    }
    pub fn add_controlled_qubit_gate(&mut self, gate: &str, control: i64, target: i64, value: Option<f64>)
    {
        let qasm_instruction = match value {
            Some(v) => format!("{}({}) q[{}], q[{}];", gate, v, control.to_string(), target.to_string()),
            None => format!("{} q[{}], q[{}];", gate, control.to_string(), target.to_string()),
        };
        self.qasm_code.push(qasm_instruction);
    }
    pub fn add_measurement(&mut self, qubit: i64) {
        self.qasm_code.push(format!("measure q[{}] -> c[{}];", qubit, qubit));
    }

    pub fn export_qasm(&self) -> String {
        let code = self.qasm_code.join("\n");
        // following might be better placed in a integration test : this test whether or not the generated qasm code is good
        let mut cache = SourceCache::new();
        let mut parser = Parser::new(&mut cache);
        parser.parse_source(code.clone(), None::<&Path>);
        match parser.done().to_errors() {
            Ok(program) => {
                if let Err(errors) = program.type_check().to_errors() {
                    godot_print!("qasm code generation error, invalid type : {:?}", errors)
                }
            },
            Err(errors) => godot_print!("qasm code generation error, invalid source : {:?}", errors),
        }
        code
    }
}
