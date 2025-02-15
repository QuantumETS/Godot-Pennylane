extends Node3D
signal theta_phi_changed(theta:float,phi:float)
func set_bloch_sphere_to_statevector(real_1,im_1,real_2,im_2):
	var theta = 2 * acos(sqrt(real_1 ** 2 + im_1 ** 2)) 
	var phi = atan2(im_2, real_2) - atan2(im_1, real_1)  
	theta_phi_changed.emit(theta,phi)
	$MeshInstance3D/Anchor_node.rotation = Vector3(theta,phi+PI/2,0.0)
