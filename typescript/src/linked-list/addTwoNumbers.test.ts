import { describe, test, expect } from "bun:test";
import { addTwoNumbers } from "./addTwoNumbers";
import { ListNode } from "./ListNode.ts";

describe("addTwoNumbers", () => {
	test("should add two numbers", () => {
		// l1 = 2 -> 4 -> 3
		const l1 = new ListNode(2, new ListNode(4, new ListNode(3)));
		// l2 = 5 -> 6 -> 4
		const l2 = new ListNode(5, new ListNode(6, new ListNode(4)));
		// result = 7 -> 0 -> 8
		const result = addTwoNumbers(l1, l2);
		expect(result).toEqual(new ListNode(7, new ListNode(0, new ListNode(8))));
	});

	test("should add two numbers", () => {
		// l1 = 0
		const l1 = new ListNode(0);
		// l2 = 0
		const l2 = new ListNode(0);
		// result = 0
		const result = addTwoNumbers(l1, l2);
		expect(result).toEqual(new ListNode(0));
	});

	test("should add two numbers", () => {
		// l1 = 9 -> 9 -> 9
		const l1 = new ListNode(9, new ListNode(9, new ListNode(9)));
		// l2 = 9 -> 9 -> 9 -> 9
		const l2 = new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9))));
		// result = 8 -> 9 -> 9 -> 0 -> 1
		const result = addTwoNumbers(l1, l2);
		expect(result).toEqual(new ListNode(8, new ListNode(9, new ListNode(9, new ListNode(0, new ListNode(1))))));
	});
});
