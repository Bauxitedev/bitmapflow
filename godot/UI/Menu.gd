extends HBoxContainer

signal menu_item_clicked

func _ready():
	$File.get_popup().connect("id_pressed", self, "file_button_pressed")
	
func file_button_pressed(id):
	match id:
		1: emit_signal("menu_item_clicked", "load_gif")
		2: emit_signal("menu_item_clicked", "load_spritesheet")
		3: emit_signal("menu_item_clicked", "load_separate_frames")		
		
		5: emit_signal("menu_item_clicked", "export_gif")
		6: emit_signal("menu_item_clicked", "export_spritesheet")
		7: emit_signal("menu_item_clicked", "export_separate_frames")		
		
		9: get_tree().quit()
		
func _process(_delta):
	
	var disabled = !ImageHolder.has_output_frames() || ImageProcessor.is_busy()
	
	for i in [5, 6, 7]:
		$File.get_popup().set_item_disabled(i, disabled)

func _on_About_pressed():
	emit_signal("menu_item_clicked", "show_about")
	
