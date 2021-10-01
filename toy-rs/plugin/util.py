import math

def swap_coords(co):
	assert len(co) == 3 or len(co) == 4

	if len(co) == 3:
		return [co.x, co.z, -co.y]
	else:
		return [co.x, co.z, -co.y, co.w]

		
def swap_coords_scale(co):
	assert len(co) == 3
	return [co.x, co.z, co.y]


def srgb_to_linear(values):
	return [srgb_channel_to_linear(value) for value in values]


def srgb_channel_to_linear(value):
	# https://en.wikipedia.org/wiki/SRGB#From_sRGB_to_CIE_XYZ
	if value <= 0.04045:
		return value / 12.92
	else:
		return math.pow((value + 0.055) / 1.055, 2.4)
