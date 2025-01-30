extends QuantumCircuit


# Called when the node enters the scene tree for the first time.
func _ready():
	init_circuit(5,6)
	x(0)
	x(2)
	h(1)
	h(3)
	var muh_measure = measure_all(5)
	print("here are the results : ")
	print(muh_measure)
	var exp = get_expectation_value('z')
	print("exp:")
	print(exp)
	var qasm_string = "
	  OPENQASM 2.0;
	  qreg q[1];
	  qreg r[1];
	  U (pi/2, 0, pi) r[0];
	"
	var result = run_qasm_str_probabilities(qasm_string, 10) # return an array
	print("qasm result : ")
	print(result)
	var result_memory = run_qasm_str_memory(qasm_string, 10) # return dictionary
	print("qasm memory : ")
	for key in result_memory.keys():
		print("sup")
		print(key,result_memory[key][0])
	print(type_string(typeof(result_memory)))
	var result_state = run_qasm_str_statevector(qasm_string, 10) # return dictionary
	print("qasm state : ")
	for key in result_state.keys():
		print(key,result_state[key])
	var result_hist = run_qasm_str_histogram(qasm_string, 100) # return dictionary
	print("qasm hist : ")
	for key in result_hist.keys():
		print(key,result_hist[key])

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
