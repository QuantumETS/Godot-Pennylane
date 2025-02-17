extends Camera3D

var radius := 10.0  
var yaw := 0.0      
var pitch := 0.0    

const SPEED := 2.0 

func _ready():
	radius = sqrt(pow(position.x,2)+pow(position.y,2)+pow(position.z,2))
	update_camera_position()

#func _physics_process(_elta: float) -> void:
	#var focus_owner = get_viewport().gui_get_focus_owner()
	#if focus_owner and Input.is_anything_pressed():
		#focus_owner.release_focus()

func _process(delta):
	var input_vector = Vector2.ZERO
	
	# WASD movement: Adjust yaw (horizontal rotation)
	if Input.is_action_pressed("move_left"):    # A
		input_vector.x += 1
	if Input.is_action_pressed("move_right"):   # D
		input_vector.x -= 1
	if Input.is_action_pressed("move_up"):      # W
		input_vector.y += 1
	if Input.is_action_pressed("move_down"):    # S
		input_vector.y -= 1
	# Update yaw & pitch based on input
	yaw += input_vector.x * SPEED * delta
	pitch = clamp(pitch + input_vector.y * SPEED * delta, -PI/2 + 0.1, PI/2 - 0.1)  # Prevent flipping
	# Update position
	update_camera_position()

func update_camera_position():
	# Convert spherical coordinates to Cartesian
	var x = radius * cos(yaw) * cos(pitch)
	var y = radius * sin(pitch)
	var z = radius * sin(yaw) * cos(pitch)

	global_transform.origin = Vector3(x, y, z)
	look_at(Vector3.ZERO)
