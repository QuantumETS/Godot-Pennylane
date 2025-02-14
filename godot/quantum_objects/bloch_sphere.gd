extends Node3D

func apply_theta_phi(theta, phi):
	$MeshInstance3D/Anchor_node.rotation = Vector3(-theta,phi,0.0)
