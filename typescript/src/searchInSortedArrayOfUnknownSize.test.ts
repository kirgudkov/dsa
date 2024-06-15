import { test, expect } from "bun:test";
import { searchInSortedArrayOfUnknownSize } from "./searchInSortedArrayOfUnknownSize";
import type { ArrayReader } from "./searchInSortedArrayOfUnknownSize";

test("searchInSortedArrayOfUnknownSize", () => {
	const reader: (arr: number[]) => ArrayReader = (arr) => ({
		get(index: number) {
			if (index > arr.length - 1) {
				return 2147483647;
			}

			return arr[index];
		}
	});

	expect(searchInSortedArrayOfUnknownSize(reader([1, 3, 5, 7, 9, 11, 13, 15, 17, 19]), 13)).toBe(6);
	expect(searchInSortedArrayOfUnknownSize(reader([1, 3, 5, 7, 9, 11, 13, 15, 17, 19]), 8)).toBe(-1);
	expect(searchInSortedArrayOfUnknownSize(reader([-1, 0, 5]), 0)).toBe(1);
});
