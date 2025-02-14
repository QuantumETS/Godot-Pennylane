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
	$"../Menu/CodeEdit".text=export_to_openqasm_string()

	var a = st["bases"][0]["re"]
	var b = st["bases"][0]["im"]
	var c = st["bases"][1]["re"]
	var d = st["bases"][1]["im"]
	
	var theta = 2 * acos(sqrt(a * a + b * b))
	var phi = atan2(d, c) - atan2(b, a) 
	$bloch_sphere.apply_theta_phi(theta, phi)
	#todo : add display of the openqasm code in realtime that can be copy and pasted
	# add button
