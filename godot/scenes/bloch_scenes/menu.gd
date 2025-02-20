extends Control


func _on_menu_button_about_to_popup() -> void:
	print("testing")


func _on_popup_menu_button_up() -> void:
	$esc_menu/gui_holder.visible = true
