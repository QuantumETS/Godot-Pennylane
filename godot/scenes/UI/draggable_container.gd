extends HBoxContainer
func _move_up():
	var parent = get_parent()
	var index = parent.get_children().find(self)
	if index > 0:  # Prevent moving above the first element
		parent.move_child(self, index - 1)
		$"../..".update_the_bloch()
