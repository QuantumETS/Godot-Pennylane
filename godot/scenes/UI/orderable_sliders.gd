extends Control
var rx = 0.0
var ry = 0.0
var rz = 0.0
var gate_order = ["rx", "ry", "rz"]
func _on_rx_slider_value_changed(value: float) -> void:
	rx = value*2*PI/360
	$"../../QuantumCircuit".update_1_qubit_circuit(rx,ry,rz,gate_order)
	$VBoxContainer/rx_box/rx_text.text = "RX(%.6f)" % (rx)


func _on_ry_slider_value_changed(value: float) -> void:
	ry = value*2*PI/360
	$"../../QuantumCircuit".update_1_qubit_circuit(rx,ry,rz,gate_order)
	$VBoxContainer/ry_box/ry_text.text = "RY(%.6f)" % (ry)

func _on_rz_slider_value_changed(value: float) -> void:
	rz = value*2*PI/360
	$"../../QuantumCircuit".update_1_qubit_circuit(rx,ry,rz,gate_order)
	$VBoxContainer/rz_box/rz_text.text = "RZ(%.6f)" % (rz)
