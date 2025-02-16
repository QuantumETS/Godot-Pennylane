extends Control
var rx = 0.0
var ry = 0.0
var rz = 0.0
var gate_order = ["rx", "ry", "rz"]
func _on_rx_slider_value_changed(value: float) -> void:
	rx = value*2*PI/360
	gate_order = get_gate_order()
	$"../../QuantumCircuit".update_1_qubit_circuit(rx,ry,rz,gate_order)
	$VBoxContainer/rx_box/rx_text.text = "RX(%.6f)" % (rx)

func _on_ry_slider_value_changed(value: float) -> void:
	ry = value*2*PI/360
	gate_order = get_gate_order()
	$"../../QuantumCircuit".update_1_qubit_circuit(rx,ry,rz,gate_order)
	$VBoxContainer/ry_box/ry_text.text = "RY(%.6f)" % (ry)

func _on_rz_slider_value_changed(value: float) -> void:
	rz = value*2*PI/360
	gate_order = get_gate_order()
	$"../../QuantumCircuit".update_1_qubit_circuit(rx,ry,rz,gate_order)
	$VBoxContainer/rz_box/rz_text.text = "RZ(%.6f)" % (rz)

func update_the_bloch() -> void:
	gate_order = get_gate_order()
	$"../../QuantumCircuit".update_1_qubit_circuit(rx,ry,rz,gate_order)

func get_gate_order() -> Array:
	var suffix = "_box"
	var matching_children = []
	for child in $VBoxContainer.get_children():
		if child.name.ends_with(suffix):
			matching_children.append(child.name.substr(0, 2))

	return matching_children
