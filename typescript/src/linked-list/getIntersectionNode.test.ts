import { describe, test, expect } from "bun:test";
import { getIntersectionNode } from "./getIntersectionNode";
import { ListNode } from "./ListNode";

describe("getIntersectionNode", () => {
	test("Example 1", () => {
		const headA = new ListNode(4);
		headA.next = new ListNode(1);
		headA.next.next = new ListNode(8);
		headA.next.next.next = new ListNode(4);
		headA.next.next.next.next = new ListNode(5);

		const headB = new ListNode(5);
		headB.next = new ListNode(0);
		headB.next.next = new ListNode(1);
		headB.next.next.next = headA.next.next;

		const expected = headA.next.next;
		const result = getIntersectionNode(headA, headB);

		expect(result).toBe(expected);
	});

	test("Example 2", () => {
		const headA = new ListNode(1);
		headA.next = new ListNode(9);
		headA.next.next = new ListNode(1);
		headA.next.next.next = new ListNode(2);
		headA.next.next.next.next = new ListNode(4);

		const headB = new ListNode(3);
		headB.next = headA.next.next.next;

		const expected = headA.next.next.next;
		const result = getIntersectionNode(headA, headB);

		expect(result).toBe(expected);
	});

	test("Example 3", () => {
		const headA = new ListNode(2);
		headA.next = new ListNode(6);
		headA.next.next = new ListNode(4);

		const headB = new ListNode(1);
		headB.next = new ListNode(5);

		const expected = null;
		const result = getIntersectionNode(headA, headB);

		expect(result).toBe(expected);
	});
});
