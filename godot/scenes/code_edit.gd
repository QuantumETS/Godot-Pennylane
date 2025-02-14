extends TextEdit

func _shortcut_input(event):
	if event is InputEventKey and event.pressed:
		if event.ctrl_pressed:
			match event.keycode:
				KEY_C:
					if has_selection():
						DisplayServer.clipboard_set(get_selected_text())
