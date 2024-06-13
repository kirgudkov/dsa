import { test, expect } from "bun:test";
import { findMaximumXOR } from "./findMaximumXOR";

test("Example 1", () => {
	expect(findMaximumXOR([3, 10, 5, 25, 2, 8])).toBe(28);
	expect(findMaximumXOR([0])).toBe(0);
	expect(findMaximumXOR([2, 4])).toBe(6);
	expect(findMaximumXOR([14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70])).toBe(127);
});
