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
    pub fn add_gate(&mut self, gate: &str, qubits: &[i64], value: Option<f64>) {
        let qasm_instruction = match value {
            Some(v) => format!("{} {}({})", gate, qubits.iter().map(|q| q.to_string()).collect::<Vec<_>>().join(","), v),
            None => format!("{} {}", gate, qubits.iter().map(|q| q.to_string()).collect::<Vec<_>>().join(",")),
        };
        self.qasm_code.push(qasm_instruction);
    }

    pub fn add_measurement(&mut self, qubit: i64) {
        self.qasm_code.push(format!("measure {}", qubit));
    }

    pub fn export_qasm(&self) -> String {
        self.qasm_code.join("\n")
    }
}
