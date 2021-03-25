extends "res://UI/Spritesheet/SpritesheetConfigBase.gd"

var img_filename setget set_img_filename
var rects = []
signal rects_changed

func _ready():
	
	var spritesheet_params_ui = {
		
		
		"frame_count": {
			"label": "Frame count",
			"ui_type": "number",
			"min": 1,
			"max": 1000,
			"default": 4
		},
		
		"frames_per_row": {
			"label": "Frames per row",
			"ui_type": "number",
			"min": 1,
			"max": 1000,
			"default": 10
		},
		
		"sep_frame_size": {
			"label": "Frame size",
			"ui_type": "header",
		},
		
		"frame_width": {
			"label": "Width",
			"ui_type": "number",
			"min": 1,
			"max": 10000,
			"default": 32
		},
		
		"frame_height": {
			"label": "Height",
			"ui_type": "number",
			"min": 1,
			"max": 10000,
			"default": 32
		},
		
		"sep_frame_offset": {
			"label": "Frame offset",
			"ui_type": "header",
		},
		
				
		"frame_offset_x": {
			"label": "Offset X",
			"ui_type": "number",
			"min": 0,
			"max": 10000,
			"default": 0
		},
		
						
		"frame_offset_y": {
			"label": "Offset Y",
			"ui_type": "number",
			"min": 0,
			"max": 10000,
			"default": 0
		},
		
		"sep_frame_margin": {
			"label": "Frame margin",
			"ui_type": "header",
		},
		
		"frame_margin_x": {
			"label": "Margin X",
			"ui_type": "number",
			"min": 0,
			"max": 10000,
			"default": 0
		},
		
		"frame_margin_y": {
			"label": "Margin Y",
			"ui_type": "number",
			"min": 0,
			"max": 10000,
			"default": 0
		},

	}
	
	setup_spritesheet_params_ui(spritesheet_params_ui)
	update_rects()
	

func set_img_filename(value):
	img_filename = value
	
	tex = ImageTexture.new()
	var img = Image.new()
	img.load(img_filename)
	tex.create_from_image(img)
	
	# NOTE, cannot use onready here since set_img_filename is called before ready()
	var spritesheet_view = get_node(spritesheet_view_path)
	
	spritesheet_view.tex = tex
	update_img_info()
	
func _on_value_changed(key, value):
	._on_value_changed(key, value)
	update_rects()

func update_rects():
	rects = []
	
	var rect_w = int(spritesheet_params.frame_width)
	var rect_h = int(spritesheet_params.frame_height)
	
	var total_rect = Rect2(0, 0,  tex.get_width(),  tex.get_height()) 
	
	var calculated_rects_per_row = tex.get_width() / rect_w
	
	var rects_on_this_row = 0
	var rect_x = spritesheet_params.frame_offset_x
	var rect_y = spritesheet_params.frame_offset_y
	
	if calculated_rects_per_row > 0:
		for i in spritesheet_params.frame_count:
			
			var rect = Rect2(rect_x, rect_y, rect_w, rect_h)
			
			if total_rect.encloses(rect):
				rects.push_back(rect)
			else:
				print("warning: rects exceed image")
				
			rect_x += rect_w + spritesheet_params.frame_margin_x
			rects_on_this_row += 1
			if rect_x + rect_w > tex.get_width() || rects_on_this_row >= spritesheet_params.frames_per_row:
				rect_x = spritesheet_params.frame_offset_x
				rect_y += rect_h + spritesheet_params.frame_margin_y
				rects_on_this_row = 0
				
	else:
		print("warning: can't fit any rect in the image")
		
	emit_signal("rects_changed", rects)	

func _on_SpritesheetView_mouse_clicked_at(pos):
	set_ui_value("frame_offset_x", clamp(round(pos.x), 0.0, tex.get_width() - spritesheet_params.frame_width))
	set_ui_value("frame_offset_y",clamp(round(pos.y), 0.0, tex.get_height() - spritesheet_params.frame_height))

func get_confirmation_error():
	if len(rects) == 0:
		return "No frames to import! Make sure your frames all fit inside the spritesheet!"
	
	return null
