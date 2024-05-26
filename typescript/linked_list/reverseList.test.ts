import { describe, test, expect } from "bun:test";
import { reverseList } from "./reverseList.ts";

describe("reverseList", () => {
	test("should reverse the linked list", () => {
		const head = { val: 1, next: { val: 2, next: { val: 3, next: { val: 4, next: { val: 5, next: null } } } } };
		const reversed = reverseList(head);

		expect(reversed?.val).toBe(5);
		expect(reversed?.next?.val).toBe(4);
		expect(reversed?.next?.next?.val).toBe(3);
		expect(reversed?.next?.next?.next?.val).toBe(2);
		expect(reversed?.next?.next?.next?.next?.val).toBe(1);
		expect(reversed?.next?.next?.next?.next?.next).toBe(null);
	});

	test("should return null if head is null", () => {
		expect(reverseList(null)).toBe(null);
	});

	test("should return the same node if there is only one node", () => {
		const head = { val: 1, next: null };
		const reversed = reverseList(head);

		expect(reversed).toBe(head);
	});
});
