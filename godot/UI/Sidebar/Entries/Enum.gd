extends "res://UI/Sidebar/Entries/EntryBase.gd"

var items setget set_items
export var default = 0 setget set_default

func _ready():
	pass
	
func set_items(values):
	for v in values:
		$OptionButton.add_item(v)

func _on_OptionButton_item_selected(index):
	emit_signal("value_changed", key, $OptionButton.get_item_text(index))

func set_value(v):
	$OptionButton.select(find_item(v))

func set_default(v):
	set_value(v)

func find_item(string):
	
	for i in $OptionButton.get_item_count():
		if $OptionButton.get_item_text(i) == string:
			return i
			
	return -1
