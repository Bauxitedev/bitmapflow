extends Control

export(ViewportTexture) var texture

signal mouse_clicked_at(pos)

func _ready():
	pass

func _process(_delta):
	update()
	
# Taken from https://github.com/godotengine/godot/blob/e8f73124a7d97abc94cea3cf7fe5b5614f61a448/scene/gui/texture_rect.cpp#L65	
func get_texture_scale() -> Rect2:
	var texture_rect = texture.get_size()
	
	var size = get_size()
	var tex_width = texture_rect.x * size.y / texture_rect.y
	var tex_height = size.y

	if (tex_width > size.x):
		tex_width = size.x
		tex_height = texture_rect.y * tex_width / texture_rect.x

	var offset = Vector2(size.x - tex_width, size.y - tex_height) / 2.0

	size.x = tex_width
	size.y = tex_height
	
	return Rect2(offset, size)
	
	
func _draw():
	var texture_rect = get_texture_scale()
	draw_texture_rect(texture, texture_rect, false)
	
func _on_TextureRect_gui_input(event):
	
	if event is InputEventMouseMotion and Input.is_mouse_button_pressed(BUTTON_LEFT):
		
			var mouse_pos_local =  get_local_mouse_position()
			
			var texture_rect: Rect2 = get_texture_scale()
			
			var tex_offset = texture_rect.position
			var tex_scale = texture_rect.size
			
			var invlerp_x = inverse_lerp(tex_offset.x, tex_offset.x + tex_scale.x, mouse_pos_local.x)
			var invlerp_y = inverse_lerp(tex_offset.y, tex_offset.y + tex_scale.y, mouse_pos_local.y)
			
			var mouse_pos_tex = texture.get_size() * Vector2(invlerp_x, invlerp_y)
			emit_signal("mouse_clicked_at", mouse_pos_tex)
