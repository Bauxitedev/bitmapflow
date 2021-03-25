extends "EntryBase.gd"

export var default = false setget set_default, get_default

func _ready():
	pass

func set_default(value):
	default = value
	$CheckBox.pressed = value
	
func get_default():
	return default

func _on_CheckBox_toggled(button_pressed):
	emit_signal("value_changed", key, button_pressed)

func set_value(value):
	$CheckBox.pressed = value
