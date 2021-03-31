extends Control

signal img_params_changed

signal loaded_gif(filename)
signal loaded_separate_frames(filenames)
signal loaded_spritesheet(filename, rects)

signal exported_separate_frames(filename)
signal exported_gif(filename, fps)
signal exported_spritesheet(filename, spritesheet_tex)

var last_dir 

var imageview_path = "MarginContainer/VBoxContainer/HBoxContainer/PanelTex/MarginContainer/ImageView"
onready var view_options = $MarginContainer/VBoxContainer/HBoxContainer/PanelTex/ViewOptions
onready var progressbar = $MarginContainer/VBoxContainer/ProgressBar
onready var framecounter = $MarginContainer/VBoxContainer/HBoxContainer/PanelTex/MarginContainer/MarginContainer/FrameCounter

# warning-ignore:return_value_discarded
func _ready():
	get_tree().connect("files_dropped", self, "_files_dropped")
	
func _process(_delta):
	var imageview = get_node(imageview_path)
	
	framecounter.text = ""
	
	if !ImageProcessor.is_busy() && progressbar.error == null:
		if ImageHolder.has_input_frames() and ImageHolder.has_output_frames():
			
			
			view_options.image_info_text = "%s → %s frames\n%s FPS → %s FPS" % [
				len(imageview.input_frames),
				len(imageview.output_frames),
				GlobalHolder.fps,
				GlobalHolder.fps * ImageHolder.get_speed_ratio()]
				
			framecounter.text = "Frame %s/%s" % [floor(imageview.current_frame)+1, len(imageview.input_frames)]
	else:
		view_options.image_info_text = ""
		
	
func add_bg_behind(dialog):
	var bg = preload("res://UI/BG.tscn").instance()
	add_child(bg)
	dialog.connect("visibility_changed", bg, "fade_out") # TODO check if visibility went to "hidden", not "shown"?
	
func clear_texture():
	get_node(imageview_path).output_frames = []
	
func show_file_dialog():
	var dialog = FileDialog.new()
	dialog.rect_min_size = Vector2(500, 500)
	dialog.access = FileDialog.ACCESS_FILESYSTEM
	add_child(dialog)
	dialog.popup_centered_ratio()
	
	if last_dir != null:
		dialog.current_dir = last_dir
		
	add_bg_behind(dialog)
		
	return dialog
	
# --------------------- #
	
func load_gif():
	var dialog = show_file_dialog()
	dialog.set_filters(PoolStringArray(["*.gif ; GIF Images"]))
	dialog.mode = FileDialog.MODE_OPEN_FILE
	
	yield(dialog, "file_selected")
	
	last_dir = dialog.current_dir
	var filename = dialog.current_path.replace("res://", "")
	emit_signal("loaded_gif", filename)
	clear_texture()	
	
func load_separate_frames():
	var dialog = show_file_dialog()
	dialog.set_filters(PoolStringArray(["*.png ; PNG Frames"]))
	dialog.mode = FileDialog.MODE_OPEN_FILES
	
	var filenames =  yield(dialog, "files_selected")
	last_dir = dialog.current_dir

	emit_signal("loaded_separate_frames", filenames)
	clear_texture()
	
func load_spritesheet():
	var dialog = show_file_dialog()
	dialog.set_filters(PoolStringArray(["*.png ; PNG Spritesheet", "*.jpg ; JPG Spritesheet"]))
	dialog.mode = FileDialog.MODE_OPEN_FILE
	
	yield(dialog, "file_selected")
	
	last_dir = dialog.current_dir
	var filename = dialog.current_path.replace("res://", "") 
	
	show_spritesheet_config_load_dialog(filename)
	
func show_spritesheet_config_load_dialog(filename):
	var spritesheet_dialog = preload("res://UI/Spritesheet/SpritesheetConfigLoad.tscn").instance() 
	spritesheet_dialog.img_filename = filename
	add_child(spritesheet_dialog)
	spritesheet_dialog.popup_centered_ratio()
	
	add_bg_behind(spritesheet_dialog)
	
	spritesheet_dialog.connect("error_occured", self, "_on_SpriteSheetConfigLoad_error_occured")
	
	yield(spritesheet_dialog, "spritesheet_confirmed")
	var rects = spritesheet_dialog.rects
	emit_signal("loaded_spritesheet", filename, rects)
	clear_texture()		
	
# --------------------- #
	
func export_gif():
	var dialog = show_file_dialog()
	dialog.set_filters(PoolStringArray(["*.gif ; GIF Image"]))
	dialog.mode = FileDialog.MODE_SAVE_FILE
	
	yield(dialog, "file_selected")
	
	last_dir = dialog.current_dir
	var filename = dialog.current_path.replace("res://", "") 
	
	var speed_ratio = ImageHolder.get_speed_ratio()
	var output_fps = GlobalHolder.fps * speed_ratio
	emit_signal("exported_gif", filename, output_fps)
	
func export_separate_frames():
	var dialog = show_file_dialog()
	dialog.set_filters(PoolStringArray(["*.png ; PNG Frames"]))
	dialog.mode = FileDialog.MODE_SAVE_FILE
	
	yield(dialog, "file_selected")
	
	last_dir = dialog.current_dir
	var filename = dialog.current_path.replace("res://", "") 
	emit_signal("exported_separate_frames", filename)
	
func export_spritesheet():
	var dialog = show_file_dialog()
	dialog.set_filters(PoolStringArray(["*.png ; PNG Spritesheet"]))
	dialog.mode = FileDialog.MODE_SAVE_FILE
	
	yield(dialog, "file_selected")
	
	last_dir = dialog.current_dir
	var filename = dialog.current_path.replace("res://", "") 
	
	show_spritesheet_config_save_dialog(filename)
	
func show_spritesheet_config_save_dialog(filename):
	var spritesheet_dialog = preload("res://UI/Spritesheet/SpritesheetConfigSave.tscn").instance() 
	add_child(spritesheet_dialog)
	spritesheet_dialog.popup_centered_ratio()
	
	add_bg_behind(spritesheet_dialog)
	
	yield(spritesheet_dialog, "spritesheet_confirmed")
	var tex = spritesheet_dialog.tex
	emit_signal("exported_spritesheet", filename, tex)
	
# --------------------- #

func show_about():
	var dialog = preload("res://UI/AboutPopup.tscn").instance() 
	add_child(dialog)
	dialog.popup_centered_ratio()
	
	add_bg_behind(dialog)
	
func _files_dropped(files, _screen):
	if len(files) == 1:
		var file = files[0]
		if file.ends_with(".gif"):
			emit_signal("loaded_gif", file)
			clear_texture()
		else:
			show_spritesheet_config_load_dialog(file)
	else:
		emit_signal("loaded_separate_frames", files)
		clear_texture()
		
	OS.move_window_to_foreground()
		
func _on_menu_item_clicked(item):
	call(item)
	
func _on_ImageHolder_image_loaded(frames):
	get_node(imageview_path).input_frames = frames

func _on_ImageProcessor_image_processed(frames):
	get_node(imageview_path).output_frames = frames

func _on_ImageProcessor_made_progress(progress):
	progressbar.value = progress

# TODO combine these 3 methods into 1
func _on_ImageProcessor_error_occured(error):
	progressbar.error = error
	
func _on_ImageHolder_error_occured(error):
	progressbar.error = error
	
func _on_SpriteSheetConfigLoad_error_occured(error):
	progressbar.error = error
	
func _on_Sidebar_img_params_changed(img_params):
	emit_signal("img_params_changed", img_params)

func _on_ViewOptions_FPS_changed(fps):
	GlobalHolder.fps = fps

func _on_ViewOptions_pixelmode_changed(pixelmode):
	get_node(imageview_path).pixelmode = pixelmode

func _on_ImageSaver_image_save_success(filenames):
	var msg
	match len(filenames):
		0: msg = "Uh this should never happen"
		1: msg = "Saved succesfully as %s" % filenames[0]
		2: msg = "Saved succesfully as %s, %s" % [filenames[0], filenames[1]]
		_: msg = "Saved succesfully as %s, %s, ..., %s" % [filenames[0], filenames[1], filenames[len(filenames) - 1]]
	progressbar.message = msg

func _on_ImageSaver_image_save_failure(error):
	progressbar.error = error
