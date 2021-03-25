extends Control

var tex setget set_texture
var rects = [] setget set_rects

signal mouse_clicked_at(pos)
	
func set_texture(val):
	tex = val
	$Viewport.size = tex.size
	$Viewport/Spritesheet.texture = tex

func set_rects(value):
	rects = value
	$Viewport/Spritesheet.rects = value

func _on_SpritesheetConfigLoad_rects_changed(value):
	self.rects = value

func _on_TextureRect_mouse_clicked_at(pos):
	emit_signal("mouse_clicked_at", pos)
