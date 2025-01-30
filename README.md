# Godot-Quantum-Simulator-Node
This repo aim to integrate existing quantum computing simulators within the godot game engine for rapid prototyping of qubits visualization and quantum computing games.

## Supported Operators

The newly added QuantumCircuit node has the following function :

### Initialization

- **`init_circuit(nb_qubits: i64, nb_bits: i64)`**  
  Initializes a quantum circuit with the specified number of qubits and classical bits.  
  - `nb_qubits`: Number of qubits in the circuit.  
  - `nb_bits`: Number of classical bits for measurement results.

### Single-Qubit Gates

- **`x(qubits_nb: i64)`**  
  Applies the Pauli-X gate (bit-flip operation) to the specified qubit.

- **`y(qubits_nb: i64)`**  
  Applies the Pauli-Y gate (combined bit-flip and phase-flip operation) to the specified qubit.

- **`z(qubits_nb: i64)`**  
  Applies the Pauli-Z gate (phase-flip operation) to the specified qubit.

- **`h(qubits_nb: i64)`**  
  Applies the Hadamard gate, creating superposition for the specified qubit.

- **`p(qubits_nb: i64, value: f64)`**  
  Applies the Phase gate with a custom angle `value` (in radians) to the specified qubit.

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

### Expectation Values

- **`get_expectation_value(measurement_axis_x_y_z: GString)`**  
  Calculates the expectation values of the circuit along a specified measurement axis:  
  - Supported axis: `"x"`, `"y"`, `"z"`.


## Installation

TBD

## Project file organisation

The files under the godot/ folder are all related directly to the godot game engine. You can import the godot project from there.

The files under godot-quantum-rust/ is where the GDextension code to create a quantum circuit node reside.

`libs.rs` is the entry point for godot. from there  `QuantumCircuit.rs` and `somethingnameSimulator.rs` are loaded.

`QuantumCircuit.rs` define the interface between godot and quantum computing simulators. It also define an interface to load different kind of simulator. When interfacing with a rust simulator, you should add your rust simulator into the dropdown enum over there and the way to construct your simulator in the match pattern code.

`SpinozaSimulator.rs` is an implementation of the QuantumSimulator trait to interface with the Spinoza simulator. Look at this file to give you an idea on what is needed to implement your own simulator. 

## How to run this repo locally/compile
go into the godot-pennylane-rust folder and use `cargo build` as you would any other rust project.

if you do not have rust installed, follow these instruction to get started : https://godot-rust.github.io/book/intro/setup.html
