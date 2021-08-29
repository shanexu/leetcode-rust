def find_median_sorted_arrays(nums1, nums2):
	nums1_length = len(nums1)
	nums2_length = len(nums2)
	if nums1_length > nums2_length:
		nums1, nums2 = nums2, nums1
		nums1_length, nums2_length = nums2_length, nums1_length
	return find_median_sorted_arrays_main(nums1, 0, nums1_length, nums2, 0, nums2_length)

def find_median_sorted_arrays_main(nums1, nums1_begin, nums1_length, nums2, nums2_begin, nums2_length):
	while True:
		print(nums1, nums1_begin, nums1_length, nums2, nums2_begin, nums2_length)
		if nums1_length == 0:
			return find_median(nums2, nums2_begin, nums2_length)
		if nums1_length == 1:
			if nums2_length == 1:
				return median2(nums1[nums1_begin], nums2[nums2_begin])
			elif nums2_length % 2 == 1:
				mid_idx = nums2_begin + nums2_length // 2
				return median2(nums2[mid_idx], median3(nums1[nums1_begin], nums2[mid_idx - 1], nums2[mid_idx + 1]))
			else:
				mid_idx = nums2_begin + nums2_length // 2
				return median3(nums1[nums1_begin], nums2[mid_idx - 1], nums2[mid_idx])
		if nums1_length == 2:
			if nums2_length == 2:
				return median4(nums1[nums1_begin], nums1[nums1_begin+1], nums2[nums2_begin], nums2[nums2_begin+1])
			elif nums2_length % 2 == 1:
				# a b x y z max(a, x), min(b, z)
				# x <= y <= z
				# a or x <= max(a, x) <= b or z
				# a or x <= min(b, z) <= b or z
				mid_idx = nums2_begin + nums2_length // 2
				return median3(max(nums1[nums1_begin], nums2[mid_idx - 1]),
							   nums2[mid_idx],
							   min(nums1[nums1_begin+1], nums2[mid_idx + 1]))
			else:
				# a b w x y z
				# w <= x, y <= z
				# a or w <= max(a, w) <= b or z
				# a or w <= min(b, z) <= b or z
				mid_idx = nums2_begin + nums2_length // 2
				return median4(max(nums1[nums1_begin], nums2[mid_idx-2]),
						nums2[mid_idx-1],
						nums2[mid_idx],
						min(nums1[nums1_begin+1], nums2[mid_idx+1]))
		nums1_mid_idx = nums1_begin + (nums1_length - 1) // 2
		nums2_mid_idx = nums2_begin + (nums2_length - 1) // 2
		if nums1[nums1_mid_idx] <= nums2[nums2_mid_idx]:
			nums1_length = nums1_length - (nums1_mid_idx - nums1_begin)
			nums2_length = nums2_length - (nums1_mid_idx - nums1_begin)
			nums1_begin = nums1_mid_idx
		else:
			nums1_length = nums1_length - (nums1_mid_idx - nums1_begin)
			nums2_length = nums2_length - (nums1_mid_idx - nums1_begin)
			nums2_begin = nums2_begin + (nums1_mid_idx - nums1_begin)

def find_median(nums, nums_begin, length):
	mid_idx = nums_begin + length // 2
	if length % 2 == 0:
		return (nums[mid_idx-1] + nums[mid_idx]) / 2.0
	else:
		return (nums[mid_idx])

def median2(a, b):
	return (a + b) / 2.0

def median3(a, b, c):
	if a > b:
		a, b = b, a
	if a > c:
		a, c = c, a
	if b > c:
		b, c = c, b
	return b

# [
# 	median3(1, 2, 3),
# 	median3(1, 3, 2),
# 	median3(2, 1, 3),
# 	median3(2, 3, 1),
# 	median3(3, 1, 2),
# 	median3(3, 2, 1)
# ]


def median4(a, b, c, d):
	if a > b:
		a, b = b, a
	if a > c:
		a, c = c, a
	if b > c:
		b, c = c, b
	if d <= a:
		return (a + b) / 2.0
	if d <= c:
		return (b + d) / 2.0
	return (b + c) / 2.0

# [
# 	median4(1, 2, 3, 4),
# 	median4(1, 2, 4, 3),
# 	median4(1, 3, 2, 4),
# 	median4(1, 3, 4, 2),
# 	median4(1, 4, 2, 3),
# 	median4(1, 4, 3, 2),
# 	median4(2, 1, 3, 4),
# 	median4(2, 1, 4, 3),
# 	median4(2, 3, 1, 4),
# 	median4(2, 3, 4, 1),
# 	median4(2, 4, 1, 3),
# 	median4(2, 4, 3, 1),
# 	median4(3, 1, 2, 4),
# 	median4(3, 1, 4, 2),
# 	median4(3, 2, 1, 4),
# 	median4(3, 2, 4, 1),
# 	median4(3, 4, 1, 2),
# 	median4(3, 4, 2, 1),
# 	median4(4, 1, 2, 3),
# 	median4(4, 1, 3, 2),
# 	median4(4, 2, 1, 3),
# 	median4(4, 2, 3, 1),
# 	median4(4, 3, 1, 2),
# 	median4(4, 3, 2, 1)
# ]

if __name__ == '__main__':
	# print(find_median_sorted_arrays([-5, 3, 6, 12, 15], [-12, -10, -6, -3, 4, 10]))
	# print(find_median_sorted_arrays([2, 3, 5, 8], [10, 12, 14, 16, 18, 20]))
	# print(find_median_sorted_arrays([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], [11, 12, 13, 14, 15, 16, 17, 18, 19]))
	# print(find_median_sorted_arrays([1, 2, 3, 4, 5], [1, 2, 3, 4, 5, 6]))
	print(find_median_sorted_arrays([1, 4, 5, 6], [2, 3, 7, 8]))
	print(find_median_sorted_arrays([1, 4, 5], [3, 7, 8]))
