@tool
extends Node3D
signal theta_phi_changed(theta:float,phi:float)
func set_bloch_sphere_to_statevector(real_1,im_1,real_2,im_2):
	var theta = 2 * acos(sqrt(real_1 ** 2 + im_1 ** 2)) 
	var phi = atan2(im_2, real_2) - atan2(im_1, real_1)  
	theta_phi_changed.emit(theta,phi)
	$sphere_mesh/Anchor_node.rotation = Vector3(theta,phi+PI/2,0.0)

@export var show_axes: bool = true:
	set(value):
		show_axes = value
		if is_inside_tree():
			_update_node_visibility("axes",!show_axes)

@export var show_labels: bool = true:
	set(value):
		show_labels = value
		if is_inside_tree():
			_update_node_visibility("axis labels",!show_labels)

@export var show_sphere: bool = true:
	set(value):
		show_sphere = value
		if is_inside_tree():
			_update_sphere_visibility("sphere_mesh",!show_sphere)

func _update_node_visibility(nodename,condition) -> void:
	for child in get_children():
		if child is Node and child.name == nodename:
			if condition:
				child.hide()
			else:
				child.show()

func _update_sphere_visibility(nodename,condition) -> void:
	for child in get_children():
		if child is Node and child.name == nodename:
			if condition:
				child.get_surface_override_material(0).set_shader_parameter("wire_color", Color("00000000"))
			else:
				child.get_surface_override_material(0).set_shader_parameter("wire_color", Color("000000aa"))
