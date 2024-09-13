interface ArrayReader {
	get(index: number): number;
}

function searchInSortedArrayOfUnknownSize(reader: ArrayReader, target: number): number {
	let l = 0;
	let r = Math.abs(target) + 1;

	while (l <= r) {
		if (reader.get(r) < target) {
			r *= 2;
		}

		const mid = l + Math.floor((r - l) / 2);

		if (reader.get(mid) == target) {
			return mid;
		}

		if (reader.get(mid) < target) {
			l = mid + 1;
		} else {
			r = mid - 1
		}
	}

	return -1;
}

export { searchInSortedArrayOfUnknownSize };
export type { ArrayReader };
