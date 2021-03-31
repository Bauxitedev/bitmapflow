extends ProgressBar

export(String) var error setget set_error
export(String) var message setget set_message

func _ready():
	set_error(null)
	set_message(null)
	
func _process(_delta):
	percent_visible = value > 0
	
func set_error(err):
	error = err
	if err == null:
		$ErrorContainer.hide()
	else:
		$ErrorContainer.show()
		$ErrorContainer/PanelContainer/Label.text = "ERROR: " + err.replace("\n", " → ")
		
		$MessageContainer.hide()
		$Tween.stop_all()
		$ErrorContainer.modulate = Color.white
		$Tween.interpolate_property($ErrorContainer, "modulate", Color.white, Color.transparent, 5.0, Tween.TRANS_SINE, Tween.EASE_IN_OUT, 8.0)
		$Tween.start()

func set_message(msg):
	message = msg
	if msg == null:
		$MessageContainer.hide()
	else:
		$MessageContainer.show()
		$MessageContainer/PanelContainer/Label.text =  msg
		
		$ErrorContainer.hide()
		$Tween.stop_all()
		$MessageContainer.modulate = Color.white
		$Tween.interpolate_property($MessageContainer, "modulate", Color.white, Color.transparent, 5.0, Tween.TRANS_SINE, Tween.EASE_IN_OUT, 8.0)
		$Tween.start()
