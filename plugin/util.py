

def swap_coords(co):
	assert len(co) == 3 or len(co) == 4

	if len(co) == 3:
		return [co.x, co.z, -co.y]
	else:
		return [co.x, co.z, -co.y, co.w]