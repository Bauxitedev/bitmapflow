extends HBoxContainer

# warning-ignore:unused_signal
signal value_changed

export var label = "Default" setget set_label
export var key = "default"

var label_path = "MarginContainer/Label"

func set_label(value):
	get_node(label_path).text = value
	label = value
	
func fade_in():
	show()
		
	# TODO fancy animation here?
	#$Tween.interpolate_property(self, "rect_min_size", Vector2(1,0), Vector2(100,100), 0.5, Tween.TRANS_SINE, Tween.EASE_IN_OUT)
	#$Tween.start()
	
func fade_out():
	hide()
	
	# TODO fancy animation here?
	#$Tween.interpolate_property(self, "rect_min_size", rect_size, rect_size * Vector2(1,0), 0.5, Tween.TRANS_SINE, Tween.EASE_IN_OUT)
	#$Tween.start()
