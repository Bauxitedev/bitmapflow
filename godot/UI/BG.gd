extends Panel

func _ready():
	$Tween.interpolate_method(self, "set_blur_amount", 0.0, 2.0, 0.3, Tween.TRANS_SINE, Tween.EASE_OUT)
	$Tween.start()
	
func set_blur_amount(amount):
	material.set_shader_param("amount", amount)

func fade_out():
	$Tween.interpolate_method(self, "set_blur_amount", 2.0, 0.0, 0.3, Tween.TRANS_SINE, Tween.EASE_OUT)
	$Tween.start()
	
	yield($Tween, "tween_all_completed")
	queue_free()
