extends ConfirmationDialog

signal spritesheet_confirmed

var tex: ImageTexture
var spritesheet_params = {}

onready var sidebar = $HBox/Panel/MarginContainer/Sidebar
var spritesheet_view_path = "HBox/Panel2/MarginContainer/SpritesheetView"
var img_info_path = "HBox/Panel2/MarginContainer/ImgInfo"

	
func setup_spritesheet_params_ui(spritesheet_params_ui):
	for key in spritesheet_params_ui:
		var value = spritesheet_params_ui[key]
		
		var instance
		
		match value["ui_type"]:
			"number":
				instance = preload("res://UI/Sidebar/Entries/NumInt.tscn").instance()
				instance.minimum = value["min"]
				instance.maximum = value["max"]
				instance.default = value["default"]
			"bool":
				instance = preload("res://UI/Sidebar/Entries/Check.tscn").instance()
				instance.default = value["default"]
			"header":
				instance = preload("res://UI/Sidebar/Entries/Header.tscn").instance()
		
		instance.key = key		
		instance.label = value["label"]
		instance.name = key
		
		instance.connect("value_changed", self, "_on_value_changed")
		
		sidebar.add_child(instance)
		
		if value.has("default"):
			spritesheet_params[key] = value["default"]
	
func _on_value_changed(key, value):
	spritesheet_params[key] = value

func set_ui_value(key, value):
	sidebar.get_node(key).set_value(value)

func _on_SpritesheetConfigBase_popup_hide():
	# IMPORTANT otherwise it'll become permanently laggy after you've imported a huge spritesheet
	queue_free()

func update_img_info():
	get_node(img_info_path).text = "Image size: %sx%s" % [tex.get_width(), tex.get_height()]
	
func get_confirmation_error():
	# Overwrite this method to disallow confirmation (e.g. if the user has selected invalid spritesheet parameters)
	return null

func _on_SpritesheetConfigBase_confirmed():
	var err = get_confirmation_error()
	if err == null:
		hide()
		emit_signal("spritesheet_confirmed")
	else:
		show_error_popup(err)
		
func show_error_popup(err):
	var err_dialog = AcceptDialog.new()
	err_dialog.dialog_text = err
	err_dialog.window_title = "Error!"
	add_child(err_dialog)
	err_dialog.popup_centered()
	
	yield(err_dialog,"popup_hide")
	err_dialog.queue_free()
	
