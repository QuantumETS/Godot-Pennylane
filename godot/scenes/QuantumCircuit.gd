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
	print(st)
