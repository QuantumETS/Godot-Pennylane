extends QuantumCircuit

# Called when the node enters the scene tree for the first time.
func _ready():
	pass

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func update_1_qubit_rxrz_circuit(rx,rz):
	init_circuit(1,1)
	rx(0,rx) # applies rx on the first qubits with value rx
	rz(0,rz) # same for rz
	var st = run_qasm_str_statevector(export_to_openqasm_string(),1) # use qasmsim to get the statevector
	$"../Menu/TextEdit".text=export_to_openqasm_string()

	var real_1 = st["bases"][0]["re"]
	var im_1 = st["bases"][0]["im"]
	var real_2 = st["bases"][1]["re"]
	var im_2 = st["bases"][1]["im"]
	 
	$bloch_sphere.set_bloch_sphere_to_statevector(real_1,im_1,real_2,im_2)
	# todo : vérifier équivalence rx(pi) et paulix, plus similaire à un ry
	# représenter la phase avec les spinors 
