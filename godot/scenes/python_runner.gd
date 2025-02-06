extends PythonRunner


func _ready():
	print(run_python_script("print('sup')"))
