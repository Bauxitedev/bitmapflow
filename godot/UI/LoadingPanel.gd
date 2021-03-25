extends Control

func _ready():
	pass

func _process(delta):
	$HBoxContainer/LoadingIcon.rect_rotation -= delta * 300.0
