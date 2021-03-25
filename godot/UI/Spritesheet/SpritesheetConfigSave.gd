extends "res://UI/Spritesheet/SpritesheetConfigBase.gd"


func _ready():
	
	var spritesheet_params_ui = {
		
		
		"frames_per_row": {
			"label": "Frames per row",
			"ui_type": "number",
			"min": 1,
			"max": 1000,
			"default": 10
		},
		

	}
	
	setup_spritesheet_params_ui(spritesheet_params_ui)
	update_spritesheet()
	
func _on_value_changed(key, value):
	._on_value_changed(key, value)
	
	update_spritesheet()
	
func update_spritesheet():
	tex = SpritesheetGenerator.generate_spritesheet(spritesheet_params)
	get_node(spritesheet_view_path).texture = tex
	update_img_info()
