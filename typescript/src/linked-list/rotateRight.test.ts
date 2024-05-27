import { describe, test, expect } from "bun:test";
import { rotateRight } from "./rotateRight.ts";
import { ListNode } from "./ListNode.ts";

describe("src/linked-list/rotateRight", () => {
	test("Example 1", () => {
		const head = new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(4, new ListNode(5)))));
		const k = 7;
		const expected = new ListNode(4, new ListNode(5, new ListNode(1, new ListNode(2, new ListNode(3)))));
		expect(rotateRight(head, k)).toEqual(expected);
	});

	test("Example 2", () => {
		const head = new ListNode(0, new ListNode(1, new ListNode(2)));
		const k = 5;
		const expected = new ListNode(1, new ListNode(2, new ListNode(0)));
		expect(rotateRight(head, k)).toEqual(expected);
	});

	test("Example 3", () => {
		const head = new ListNode(1);
		const k = 1;
		expect(rotateRight(head, k)).toEqual(new ListNode(1));
	});

	test("Example 4", () => {
		const head = new ListNode(1, new ListNode(2));
		const k = 2;
		expect(rotateRight(head, k)).toEqual(new ListNode(1, new ListNode(2)));
	});
});
