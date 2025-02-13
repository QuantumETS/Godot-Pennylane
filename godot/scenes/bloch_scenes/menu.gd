extends Control


func _on_h_slider_value_changed(value: float) -> void:
	$rx_text.text = "RX(%s)" % (value*2*PI/360)


func _on_v_slider_value_changed(value: float) -> void:
	$rz_text.text = "RZ(%s)" % (value*2*PI/360)
