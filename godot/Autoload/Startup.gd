extends Node

func _ready():
	var screen_size = OS.get_screen_size(0)
	OS.window_size = screen_size * 0.75
	
	var window_size = OS.get_window_size()
	OS.set_window_position(screen_size*0.5 - window_size*0.5)
	
	check_rustlib()
	
func check_rustlib():
	# Check if the user extracted the zip file properly
	
	var library_file = File.new()
	var library_path = preload("res://RustLibrary.tres").get_current_library_path()
	
	if !library_file.file_exists(library_path):
		OS.alert("ERROR: Failed to find %s! Did you extract the zip file before running Bitmapflow?" % library_path);
		get_tree().quit()
	else:
		Logger.info("RustLibrary %s found" % library_path)
