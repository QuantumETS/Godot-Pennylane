@tool
extends Node3D
signal theta_phi_changed(theta:float,phi:float)

var reference_statevector = Vector4(1.0, 0.0, 0.0, 0.0)  # Default to |0⟩ state: (1, 0, 0, 0)
var last_statevector = Vector4(1.0, 0.0, 0.0, 0.0)
func set_bloch_sphere_to_statevector(real_1,im_1,real_2,im_2):
	var theta = 2 * acos(sqrt(real_1 ** 2 + im_1 ** 2)) 
	var phi = atan2(im_2, real_2) - atan2(im_1, real_1) 
	var material = $sphere_mesh/phi_mesh.mesh.material as ShaderMaterial
	material.set_shader_parameter("theta_end", phi)
	theta_phi_changed.emit(theta,phi)
	$sphere_mesh/Anchor_node.rotation = Vector3(theta,phi+PI/2,0.0)
	last_statevector = Vector4(real_1, im_1, real_2, im_2)
	# global phase representation
	var alpha = atan2(im_1, real_1)
	$sphere_mesh/Anchor_node/flag_anchor.rotation = -$sphere_mesh/Anchor_node.rotation # cancel out bad rotation
	$sphere_mesh/Anchor_node/flag_anchor.rotation = Vector3(0,compute_alpha(real_1, im_1, real_2, im_2),0)
	$sphere_mesh/global_phase_vector.rotation = Vector3(0,compute_alpha(real_1, im_1, real_2, im_2),0)


func set_current_as_reference_statevector(): # use this to compute global phase
	reference_statevector = last_statevector
	set_bloch_sphere_to_statevector(reference_statevector.x,reference_statevector.y,reference_statevector.z,reference_statevector[3])
	
# chatgpt made this function, need to confirm that it makes sense. see https://arxiv.org/pdf/1312.3824 page 3
func compute_alpha(real_1, im_1, real_2, im_2):
	var ref_real_1 = reference_statevector.x
	var ref_im_1 = reference_statevector.y
	var ref_real_2 = reference_statevector.z
	var ref_im_2 = reference_statevector.w

	# Compute the inner product ⟨ψ_0 | ψ_f⟩
	var dot_real = ref_real_1 * real_1 + ref_im_1 * im_1 + ref_real_2 * real_2 + ref_im_2 * im_2
	var dot_im = ref_real_1 * im_1 - ref_im_1 * real_1 + ref_real_2 * im_2 - ref_im_2 * real_2

	# Extract the phase (global phase α)
	return atan2(dot_im, dot_real)
########### option to show or hide various nodes of the bloch sphere scene only if it's inside another scene ###################
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

@export var show_flag: bool = true:
	set(value):
		show_flag = value
		if is_inside_tree():
			$sphere_mesh/Anchor_node/flag_anchor.set_visible(value) # too lazy to make it consistent with the rest
			
@export var show_global_phase_vec: bool = true:
	set(value):
		show_global_phase_vec = value
		if is_inside_tree():
			$sphere_mesh/global_phase_vector.set_visible(value) # too lazy to make it consistent with the rest
			
func _update_node_visibility(nodename,condition,recursive=false) -> void:
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
