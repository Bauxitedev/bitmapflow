extends Control

#TextureRect that supports integer scaling for pixel art

export var use_integer_scaling = false
export(Texture) var texture

func _ready():
	pass

func _process(_delta):
	update()
	
func _draw():
	if texture != null:
		
		# Code adapted from https://github.com/godotengine/godot/blob/e8f73124a7d97abc94cea3cf7fe5b5614f61a448/scene/gui/texture_rect.cpp#L65
		
		var tex_width = texture.get_width() * rect_size.y / texture.get_height()
		var tex_height = rect_size.y

		if tex_width > rect_size.x:
			tex_width = rect_size.x
			tex_height = texture.get_height() * rect_size.x / texture.get_width()
		
		var size = Vector2(tex_width, tex_height)
		
		if use_integer_scaling:
			var ratio = max(1, floor(size.x / float(texture.get_width())))
			size = Vector2(texture.get_width() * ratio, texture.get_height() * ratio)
		
		var offset = Vector2(rect_size.x - size.x, rect_size.y - size.y) / 2
				
		draw_texture_rect(texture, Rect2(offset, size), false)
