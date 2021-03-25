extends Sprite

var rects = []

func _process(_delta):
	update()
	
func _draw():
	var i = 0
	for rect in rects:
		var alpha = 0.6 + sin(i * 0.5 - OS.get_ticks_msec() / 100.0) * 0.4
		var color = Color(1.0, 0.1, 0.1, alpha)
		var color_shadow = Color(0.0, 0.0, 0.0, alpha * 0.5)
		
		rect = rect.grow_individual(-1, -1, 0, 0)
		draw_rect(rect, color_shadow, false, 2.0, false)
		draw_rect(rect, color, false, 1.0, false)
		
		i += 1
