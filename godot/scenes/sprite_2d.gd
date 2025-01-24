extends Player


# Called when the node enters the scene tree for the first time.
func _ready():
	print(get_test_string())


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	increase_speed(delta)
	print(get_speed())
	print(get_test_string())


func _on_speed_increased():
	pass # Replace with function body.
	print("supge")
