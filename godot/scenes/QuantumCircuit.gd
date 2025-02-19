extends QuantumCircuit

func update_statevector_text(real1,im1,real2,im2):
	var plus0 = "+" # stuff to make string formatting prettier
	var plus1 = "+" 
	var plus2 = "+"
	if im1 < 0: 
		plus0 = "-"
		im1 = abs(im1)
	if real2 < 0: 
		plus1 = "-"
		real2 = abs(real2)
	if im2 < 0: 
		plus2 = "-"
		im2 = abs(im2)
	$"../Menu/statevector_txt".text = "Statevector : %.2f %s %.2fi ∣0⟩ %s %.2f %s %.2fi ∣1⟩" % [real1,plus0,im1,plus1,real2,plus2,im2]

func update_1_qubit_circuit(rx,ry,rz,gate_order):
	init_circuit(1,1)
	for item in gate_order:
		if item == "rx":
			rx(0,rx)
		elif item == "ry":
			ry(0,ry)
		elif item == "rz":
			rz(0,rz)
	var st = run_qasm_str_statevector(export_to_openqasm_string(),1) # use qasmsim to get the statevector
	$"../Menu/TextEdit".text=export_to_openqasm_string()

	var real_1 = st["bases"][0]["re"]
	var im_1 = st["bases"][0]["im"]
	var real_2 = st["bases"][1]["re"]
	var im_2 = st["bases"][1]["im"]
	update_statevector_text(real_1,im_1,real_2,im_2)
	$bloch_sphere.set_bloch_sphere_to_statevector(real_1,im_1,real_2,im_2)
	# todo :
	# créer un menu pour cacher différentes élément de la scene + retour au menu principale


func _on_bloch_sphere_theta_phi_changed(theta: float, phi: float) -> void:
	if phi < 0.0: # this is a quick hack, for some reason phi can reach below 0 in specific config ??
		phi += 2*PI
	$"../Menu/statevector_txt/phi_theta_txt".text = "ϕ = %.2f ; θ = %.2f" % [phi, theta]
