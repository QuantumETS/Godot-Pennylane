extends Node


func _input(event):
	if event is InputEventKey and event.pressed and event.keycode == KEY_ESCAPE:
		$gui_holder.visible = !$gui_holder.visible


func _on_back_to_scene_button_up() -> void:
	$gui_holder.visible = false


func _on_axes_check_box_toggled(toggled_on: bool) -> void:
	for node in get_tree().get_nodes_in_group("bloch_sphere"):
		node.show_axes = toggled_on


func _on_labels_check_box_toggled(toggled_on: bool) -> void:
	for node in get_tree().get_nodes_in_group("bloch_sphere"):
		node.show_labels = toggled_on


func _on_sphere_check_box_toggled(toggled_on: bool) -> void:
	for node in get_tree().get_nodes_in_group("bloch_sphere"):
		node.show_sphere = toggled_on


func _on_flag_check_box_toggled(toggled_on: bool) -> void:
	for node in get_tree().get_nodes_in_group("bloch_sphere"):
		node.show_flag = toggled_on


func _on_global_phase_vec_check_box_toggled(toggled_on: bool) -> void:
	for node in get_tree().get_nodes_in_group("bloch_sphere"):
		node.show_global_phase_vec = toggled_on


func _on_language_item_selected(index: int) -> void:
	if index == 1:
		TranslationServer.set_locale("en")
	if index == 0:
		TranslationServer.set_locale("fr")
