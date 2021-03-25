extends MarginContainer

signal img_params_changed

var img_params = {}
var img_params_default = {}
var optflow_items = ["SimpleFlow", "DenseRLOF"]
var img_params_ui

onready var vbox = $VBox

func _ready():
	
	img_params_ui = {
		
		"sep_basic": {
			"label": "Basic",
			"ui_type": "header",
		},
		
		"inbetweens": {
			"label": "Inbetweens",
			"ui_type": "int",
			"min": 1,
			"max": 10,
			"default": 1
		},

		"loop_seamlessly": {
			"label": "Loop seamlessly",
			"ui_type": "bool",
			"default": true
		},
		
		"flow_multiplier": {
			"label": "Motion multiplier",
			"ui_type": "float",			
			"min": 0.01,
			"max": 10,
			"default": 1,
			"suffix": "x",
		},
		
		"sep_adv": {
			"label": "Advanced",
			"ui_type": "header",
		},
		
		"optflow_alg": {
			"label": "Flow algorithm",
			"ui_type": "enum",
			"items": optflow_items,
			"default": optflow_items[0]
		},
		
		"layers": {
			"label": "Layers",
			"ui_type": "int",
			"min": 1,
			"max": 10,
			"default": 3,
			"belongs_to": optflow_items[0]
		},
		
		"averaging_block_size": {
			"label": "Averaging block size",
			"ui_type": "int",
			"min": 1,
			"max": 10,
			"default": 2,
			"belongs_to": optflow_items[0]
		},
		
		"max_flow": {
			"label": "Max flow",
			"ui_type": "int",
			"min": 1,
			"max": 20,
			"default": 4,
			"belongs_to": optflow_items[0]
		},
		
		"grid_step_x": {
			"label": "Grid step X",
			"ui_type": "int",
			"min": 1,
			"max": 60,
			"default": 3,
			"belongs_to": optflow_items[1]
		},
		
		"grid_step_y": {
			"label": "Grid step Y",
			"ui_type": "int",
			"min": 1,
			"max": 60,
			"default": 3,
			"belongs_to": optflow_items[1]
		},
		
		"forward_backward_threshold": {
			"label": "Forward backward threshold",
			"ui_type": "float",
			"min": 0.0,
			"max": 30.0,
			"default": 0,
			"step": 0.0001,
			"belongs_to": optflow_items[1]
		},
		
		"use_post_proc": {
			"label": "Use post processing",
			"ui_type": "bool",
			"default": true,
			"belongs_to": optflow_items[1]
		},
				
		"use_variational_refinement": {
			"label": "Use variational refinement",
			"ui_type": "bool",
			"default": false,
			"belongs_to": optflow_items[1]
		},
		
		"sep_advadv": {
			"label": "Super Advanced",
			"ui_type": "header",
		},
		
		"show_motion_vectors": {
			"label": "Show motion vectors",
			"ui_type": "bool",
			"default": false
		},
	}
	
	for key in img_params_ui:
		var value = img_params_ui[key]
		
		var instance
		
		match value["ui_type"]:
			"int":
				instance = preload("res://UI/Sidebar/Entries/NumInt.tscn").instance()
				instance.minimum = value["min"]
				instance.maximum = value["max"]
			"float":
				instance = preload("res://UI/Sidebar/Entries/NumFloat.tscn").instance()
				instance.minimum = value["min"]
				instance.maximum = value["max"]
			"bool":
				instance = preload("res://UI/Sidebar/Entries/Check.tscn").instance()
			"enum":
				instance = preload("res://UI/Sidebar/Entries/Enum.tscn").instance()
				instance.items = value["items"]
			"header":
				instance = preload("res://UI/Sidebar/Entries/Header.tscn").instance()
				
		
		instance.name = key		
		instance.key = key		
		instance.label = value["label"]
		
		if "suffix" in value:
			instance.suffix = value["suffix"]
			
		if "default" in value:
			instance.default = value["default"]
			
		if "step" in value:
			instance.step = value["step"]
		
		instance.connect("value_changed", self, "_on_value_changed")
		
		vbox.add_child(instance)
		
		if value.has("default"):
			img_params[key] = value["default"]
		
	
	img_params_default = img_params.duplicate(true)
	
	update_enum_visibility("optflow_alg")
	emit_img_params_changed()

func _on_value_changed(key, value):
	img_params[key] = value
	
	if img_params_ui[key]["ui_type"] == "enum":
		update_enum_visibility(key)
	emit_img_params_changed()
	
func update_enum_visibility(enum_key):
	
	var enum_kind = img_params[enum_key]
	
	for child in vbox.get_children():
		var key = child.key
		var value = img_params_ui[key]
		
		if value.has("belongs_to"):
			var belongs_to = value["belongs_to"]
			
			var is_visible = belongs_to == enum_kind
			
			if is_visible:
				child.fade_in()
			else:
				child.fade_out()
		
func emit_img_params_changed():
	emit_signal("img_params_changed", img_params)
	

func _on_Reset_pressed():
	img_params = img_params_default.duplicate(true)
	update_ui()
	
func update_ui():
	# Update the ui elements if img_params was changed from code
	# NOTE: if a ui element refuses to be updated, check if it has a set_value method
	for child in vbox.get_children():
		
		var key = child.key
		
		if "default" in child:
			child.set_value(img_params[key])
			
		var val = img_params_ui[key]
		
		if val["ui_type"] == "enum":
			update_enum_visibility(key)


