# Godot-Quantum-Simulator-Node
This repo aim to integrate existing quantum computing simulators within the godot game engine for rapid prototyping of qubits visualization and quantum computing games.
It exposes a custom **QuantumCircuit** node with a unified interface that can drive different rust simulators (such as Spinoza, Q1tSim, etc.). It supports exportation to OpenQASM 2.0 as well as simulation from an OpenQASM string.

## Supported Operators

The **QuantumCircuit** node provides a comprehensive interface to build and simulate quantum circuits. Below is an overview of its supported functions and operators:

### Initialization

- **`init_circuit(nb_qubits: i64, nb_bits: i64)`**  
  Initializes a quantum circuit with the specified number of qubits and classical bits.  
  - `nb_qubits`: Number of qubits in the circuit.  
  - `nb_bits`: Number of classical bits for measurement results.
  - Also initializes the QASM exporter (using the same number of measurement bits as qubits by default).

### Single-Qubit Gates

- **`x(qubits_nb: i64)`**  
  Applies the Pauli-X gate (bit-flip operation) to the specified qubit.

- **`y(qubits_nb: i64)`**  
  Applies the Pauli-Y gate (combined bit-flip and phase-flip operation) to the specified qubit.

- **`z(qubits_nb: i64)`**  
  Applies the Pauli-Z gate (phase-flip operation) to the specified qubit.

- **`h(qubits_nb: i64)`**  
  Applies the Hadamard gate, creating superposition for the specified qubit.

- **`s(qubits_nb: i64, value: f64)`**  
  Applies the Phase gate with an angle of pi/2 (in radians) to the specified qubit.

- **`p(qubits_nb: i64, value: f64)`**  
  Applies the Phase gate with a custom angle (in radians) to the specified qubit.  
  *Note:* For the Spinoza simulator, this function behaves similarly to `rz` for most other simulators.  
  The operation is logged in the QASM exporter.

- **`rx(qubits_nb: i64, value: f64)`**  
  Applies the rotation-X gate with angle `value` to the specified qubit.

- **`ry(qubits_nb: i64, value: f64)`**  
  Applies the rotation-Y gate with angle `value` to the specified qubit.

- **`rz(qubits_nb: i64, value: f64)`**  
  Applies the rotation-Z gate with angle `value` to the specified qubit.

- **`identity(qubits_nb: i64)`**  
  Applies the Identity gate (does nothing but maintains circuit state).

### Two-Qubit Gates

- **`swap(qubits_nb_1: i64, qubits_nb_2: i64)`** *(Not fully implemented)*  
  Swaps the quantum states of the two specified qubits.

- **`cnot(control_qubit_nb: i64, target_qubit_nb: i64)`**  
  Applies a Controlled-NOT (CNOT) gate where the target qubit is flipped if the control qubit is in the |1⟩ state.

- **`custom_controlled(control_qubit_nb: i64, target_qubit_nb: i64, gate_name: GString, value: f64)`**  
  Applies a custom controlled gate. Supported gates:  
  - `"x"`, `"y"`, `"z"`, `"rx"`, `"ry"`, `"rz"`, `"h"`, `"p"`  
  - `value`: Angle or phase for rotation/phase gates.

### Measurements

- **`add_measurement(qubits_nb: i64)`**  
  Adds a measurement operation to the specified qubit.

- **`measure_all() -> Array<u8>`**  
  Measures all qubits in the circuit and returns the results as an array of binary values (`u8`).

### Expectation Values and State Vector

- **`get_expectation_value(measurement_axis_x_y_z: GString) -> Array<f64>`**  
  Computes the expectation value of the circuit along a specified axis.  
  - Supported axes: `"x"`, `"y"`, and `"z"`.

- **`get_statevector() -> Dictionary`**  
  Returns the current state vector of the circuit (if available) as a Godot Dictionary.

### QASM Import/Export and Simulation

- **`run_qasm_str_probabilities(qasm_string: GString, shots: i64) -> Array<f64>`**  
  Runs a QASM program and returns the outcome probabilities.

- **`run_qasm_str_memory(qasm_string: GString, shots: i64) -> Dictionary`**  
  Runs a QASM program and returns a memory map of measurement results.

- **`run_qasm_str_statevector(qasm_string: GString, shots: i64) -> Dictionary`**  
  Runs a QASM program and retrieves the state vector.

- **`run_qasm_str_histogram(qasm_string: GString, shots: i64) -> Dictionary`**  
  Runs a QASM program and computes a measurement histogram.

- **`export_to_openqasm_string() -> GString`**  
  Exports the current quantum circuit as an OpenQASM program in string format.  
  This string reflects all the operations recorded by the QASM exporter.

- **`export_to_openqasm_file_w_dialog()`**  
  Opens a file dialog (using Godot’s built-in or native dialog) to prompt the user for a save location, ensures the file name ends with `.qasm`, and writes the OpenQASM program to that file.  
  The dialog is automatically removed from the scene after use.

- **`export_to_openqasm_file(path: GString)`**  
  Exports the OpenQASM program directly to a file at the specified path.  
  If the path does not already end with `.qasm`, the extension is appended.

## Usage in Godot

1. **Adding the Node:**  
   After compiling the GDExtension, add the **QuantumCircuit** node to your scene.

2. **Building a Circuit:**  
   - Call `init_circuit()` first to initialize a new circuit.  
   - Use the various gate functions (e.g., `x()`, `h()`, `cnot()`, etc.) to build your circuit.

3. **Exporting the Circuit:**  
   - **As a String:** Use `export_to_openqasm_string()` to retrieve the OpenQASM program as a string.  
   - **To a File with Dialog:** Call `export_to_openqasm_file_w_dialog()` to open a file dialog and save the QASM file interactively.  
   - **Direct File Export:** Use `export_to_openqasm_file(path)` by providing a file path.

4. **Running QASM Programs:**  
   The node supports running QASM strings on the qasmsim simulator and retrieving different outputs such as probabilities, memory, state vector, or histograms.

---

## Project file organisation

The files under the godot/ folder are all related directly to the godot game engine. You can import the godot project from there.

The files under godot-quantum-rust/ is where the GDextension code to create a quantum circuit node reside.

`libs.rs` is the entry point for godot. from there  `QuantumCircuit.rs` and `somethingnameSimulator.rs` are loaded.

`QuantumCircuit.rs` define the interface between godot and quantum computing simulators. It also define an interface to load different kind of simulator. When interfacing with a rust simulator, you should add your rust simulator into the dropdown enum over there and the way to construct your simulator in the match pattern code.

`SpinozaSimulator.rs` is an implementation of the QuantumSimulator trait to interface with the Spinoza simulator. Look at this file to give you an idea on what is needed to implement your own simulator. (https://github.com/QuState/spinoza)

`q1tsimSimulator.rs` same but for q1tsim (https://github.com/Q1tBV/q1tsim)

`QasmExporter.rs` Integrated within the QuantumCircuit node, it logs every operation (gates, measurements, etc.) and constructs a complete OpenQASM program.

## How to run this repo locally/compile
Go into the godot-pennylane-rust folder and use `cargo build` as you would any other rust project. This will create the necessary gdextension files.  
From there you can open the project.godot file using the latest godot version available (beyond 4.4 is recommended). 

If everything works correctly, you should see a QuantumCircuit node available when creating new nodes in Godot. 

If you do not have rust installed, follow these instruction to get started : https://godot-rust.github.io/book/intro/setup.html

For now, to work on different godot projects, you have to include and possibly modify the *.gdextension file in the **godot/** folder to make them point toward the files that were compiled. In the future, the goal is to only need a single click in the asset library for it to work.
