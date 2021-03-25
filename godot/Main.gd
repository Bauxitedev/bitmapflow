# warning-ignore-all:return_value_discarded
extends Node

func _enter_tree():
	
	# Since singleton signals cannot be connected in the GUI, we need to do this via script
	
	# NOTE: this needs to be done in _enter_tree(), NOT ready().
	# This is because some children emit a signal in their ready() callback.
	# And children's ready() is called before this ready().
	# So, that would cause the signal to be sent BEFORE the signal is connected to anything.
	# So nothing happens.
	
	ImageHolder.connect("image_loaded", ImageProcessor, "_on_imageholder_image_loaded")
	ImageHolder.connect("image_loaded", $UI, "_on_ImageHolder_image_loaded")
	
	ImageProcessor.connect("image_processed", ImageHolder, "_on_imageprocessor_image_processed")
	ImageProcessor.connect("error_occured",   $UI,         "_on_ImageProcessor_error_occured")
	ImageProcessor.connect("image_processed", $UI,         "_on_ImageProcessor_image_processed")
	ImageProcessor.connect("made_progress",   $UI,         "_on_ImageProcessor_made_progress")
	
	ImageSaver.connect("image_save_failure", $UI, "_on_ImageSaver_image_save_failure")
	ImageSaver.connect("image_save_success", $UI, "_on_ImageSaver_image_save_success")

	$UI.connect("loaded_gif",               ImageHolder,    "_on_ui_loaded_gif")
	$UI.connect("loaded_separate_frames",   ImageHolder,    "_on_ui_loaded_separate_frames")
	$UI.connect("loaded_spritesheet",       ImageHolder,    "_on_ui_loaded_spritesheet")
	$UI.connect("img_params_changed",       ImageProcessor, "_on_ui_img_params_changed")
	$UI.connect("exported_gif",             ImageSaver,     "_on_ui_exported_gif")
	$UI.connect("exported_separate_frames", ImageSaver,     "_on_ui_exported_separate_frames")
	$UI.connect("exported_spritesheet",     ImageSaver,     "_on_ui_exported_spritesheet")
