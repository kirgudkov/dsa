import { test, expect } from "bun:test";
import { KthLargest } from "./KthLargest";

test("KthLargest 1", () => {
	const kthLargest = new KthLargest(3, [4, 5, 8, 2]);

	expect(kthLargest.add(3)).toBe(4);
	expect(kthLargest.add(5)).toBe(5);
	expect(kthLargest.add(10)).toBe(5);
	expect(kthLargest.add(9)).toBe(8);
	expect(kthLargest.add(4)).toBe(8);
});

test("KthLargest 2", () => {
	const kthLargest = new KthLargest(1, []);

	expect(kthLargest.add(-3)).toBe(-3);
	expect(kthLargest.add(-2)).toBe(-2);
	expect(kthLargest.add(-4)).toBe(-2);
	expect(kthLargest.add(0)).toBe(0);
	expect(kthLargest.add(4)).toBe(4);
});
