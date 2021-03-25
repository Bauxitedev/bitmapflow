extends HBoxContainer

var smooth_flags = ImageTexture.FLAG_FILTER

onready var texrectA = $HBoxA/MarginContainer/Panel/MarginContainer/TexRect
onready var texrectB = $HBoxB/MarginContainer/Panel/MarginContainer/TexRect
onready var loading_panel = $HBoxB/MarginContainer/Panel/MarginContainer/MarginContainer/LoadingPanel

var input_frames setget set_input_frames
var output_frames setget set_output_frames
var current_frame = 0
var pixelmode setget set_pixelmode

func _ready():
	pass

func _process(delta):
	
	loading_panel.visible = ImageProcessor.is_busy()
	
	if input_frames != null && !input_frames.empty():
		current_frame = fmod(current_frame + delta * GlobalHolder.fps, len(input_frames))
		display_frameA(input_frames[floor(current_frame)])
		if output_frames != null && !output_frames.empty():
			var speed_multiplier = len(output_frames) / float(len(input_frames))
			display_frameB(output_frames[min(floor(current_frame * speed_multiplier), len(output_frames)-1)])
		else:
			display_frameB(null)
	
func set_input_frames(frames):
	input_frames = frames
	update_pixelmode_input_frames()
	
func set_output_frames(frames):
	output_frames = frames
	update_pixelmode_output_frames()
	
func display_frameA(frame):
	texrectA.texture = frame	
	
func display_frameB(frame):
	texrectB.texture = frame

func set_pixelmode(value):
	pixelmode = value

	if input_frames != null:
		update_pixelmode_input_frames()
	if output_frames != null:
		update_pixelmode_output_frames()
	
	texrectA.use_integer_scaling = pixelmode
	texrectB.use_integer_scaling = pixelmode
		
# TODO this is actually kinda slow! Better way to do it would be to use a shader that manually does the pixelization effect.
func update_pixelmode_input_frames():
	for i in len(input_frames):	
		var frame = input_frames[i]
		if frame is ImageTexture:
			input_frames[i] = ImageTexture.new()
			input_frames[i].create_from_image(frame.image, 0 if pixelmode else smooth_flags)
			
func update_pixelmode_output_frames():
	for i in len(output_frames):
		var frame = output_frames[i]
		if frame is ImageTexture:
			output_frames[i] = ImageTexture.new()
			output_frames[i].create_from_image(frame.image, 0 if pixelmode else smooth_flags)
