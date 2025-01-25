extends QuantumCircuit


# Called when the node enters the scene tree for the first time.
func _ready():
	init_circuit(5,1)
	x(0)
	x(2)
	h(1)
	h(3)
	var muh_measure = measure_all()
	print("here are the results : ")
	print(muh_measure)
	get_expectation_value('z')


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
