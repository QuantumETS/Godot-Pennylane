use godot::prelude::*;

mod QuantumCircuit; // libs.rs is godot's entry point, mod QuantumCircuit make our QuantumCircuit visible to godot
mod SpinozaSimulator;


struct PennylaneGodotRust;

#[gdextension]
unsafe impl ExtensionLibrary for PennylaneGodotRust {}