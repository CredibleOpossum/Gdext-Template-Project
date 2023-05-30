extends Node

func _process(_delta):
	self.text = str(Engine.get_frames_per_second());
