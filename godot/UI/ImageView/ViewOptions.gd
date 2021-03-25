extends Panel

signal FPS_changed
signal pixelmode_changed

var image_info_text setget set_image_info_text

func _ready():
	_on_FPS_value_changed($VBox/FPS.value)
	_on_Pixelmode_toggled($VBox/Pixelmode.pressed)
	
func _on_FPS_value_changed(value):
	emit_signal("FPS_changed", value)

func _on_Pixelmode_toggled(value):
	emit_signal("pixelmode_changed", value)
	
func set_image_info_text(v):
	$VBox/ImageInfo.text = v
