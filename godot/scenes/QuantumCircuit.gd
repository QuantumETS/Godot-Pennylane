extends QuantumCircuit


# Called when the node enters the scene tree for the first time.
func _ready():
	init_circuit(5,1)
	x(0)
	x(1)
	measure()


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
