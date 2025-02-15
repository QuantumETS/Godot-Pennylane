extends QuantumCircuit

func update_statevector_text(real1,im1,real2,im2):
	# im1 will always be 0
	var plus1 = "+" # stuff to make string formatting prettier
	var plus2 = "+"
	if real2 < 0: 
		plus1 = "-"
		real2 = abs(real2)
	if im2 < 0: 
		plus2 = "-"
		im2 = abs(im2)
	$"../Menu/statevector_txt".text = "Statevector : %.2f ∣0⟩ %s %.2f %s %.2fi ∣1⟩" % [real1,plus1,real2,plus2,im2]

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
	update_statevector_text(real_1,im_1,real_2,im_2)
	$bloch_sphere.set_bloch_sphere_to_statevector(real_1,im_1,real_2,im_2)
	# todo : vérifier équivalence rx(pi) et paulix, plus similaire à un ry
	# représenter la phase avec les spinors 
	# afficher theta et phi sur le bloch sphere
	# afficher l'ordre du circuit simuler et donner la possibilité de changer l'ordre
	# ajouter ry
	
