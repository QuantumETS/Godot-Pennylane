extends Control
var rx = 0.0
var rz = 0.0

func _on_h_slider_value_changed(value: float) -> void:
	rx = value*2*PI/360
	$rx_text.text = "RX(%s)" % (rx)
	$"../QuantumCircuit".update_1_qubit_rxrz_circuit(rx,rz)


func _on_v_slider_value_changed(value: float) -> void:
	rz = value*2*PI/360
	$rz_text.text = "RZ(%s)" % (rz)
	$"../QuantumCircuit".update_1_qubit_rxrz_circuit(rx,rz)
