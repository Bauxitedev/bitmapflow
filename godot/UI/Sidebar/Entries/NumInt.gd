extends "EntryBase.gd"


export var minimum = 10 setget set_minimum
export var maximum = 100 setget set_maximum
export var default = 75 setget set_default, get_default
export var step = 1 setget set_step
export var suffix = "" setget set_suffix



func set_minimum(value):
	$TextEdit.min_value = value
	
func set_maximum(value):
	$TextEdit.max_value = value
	
func set_default(value):
	default = value
	$TextEdit.value = value
	
func get_default():
	return default
	
func set_step(s):
	$TextEdit.step = s
	
func set_suffix(value):
	$TextEdit.suffix = value
	
func _on_TextEdit_value_changed(value):
	emit_signal("value_changed", key, value)

func set_value(val):
	$TextEdit.value = val

