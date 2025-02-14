extends Node3D

const AXIS_THICKNESS = 0.005
const AXIS_LENGTH = 1.0

func _ready():
	var x_axis = create_axis(Vector3.RIGHT * (AXIS_LENGTH / 2), AXIS_LENGTH, Color.RED, Vector3(0, 0, deg_to_rad(90)))
	var y_axis = create_axis(Vector3.FORWARD * (AXIS_LENGTH / 2), AXIS_LENGTH, Color.GREEN, Vector3(deg_to_rad(90), 0, 0))
	var z_axis = create_axis(Vector3.UP * (AXIS_LENGTH / 2), AXIS_LENGTH, Color.BLUE, Vector3.ZERO)

	add_child(x_axis)
	add_child(y_axis)
	add_child(z_axis)

func create_axis(pos: Vector3, length: float, color: Color, rot: Vector3) -> MeshInstance3D:
	var mesh_instance = MeshInstance3D.new()
	var mesh = CylinderMesh.new()
	
	mesh.top_radius = AXIS_THICKNESS / 2
	mesh.bottom_radius = AXIS_THICKNESS / 2
	mesh.height = length

	var mat = StandardMaterial3D.new()
	mat.albedo_color = color
	mat.shading_mode = BaseMaterial3D.SHADING_MODE_UNSHADED  

	mesh_instance.mesh = mesh
	mesh_instance.material_override = mat
	mesh_instance.transform.origin = pos
	mesh_instance.rotation = rot 

	return mesh_instance
