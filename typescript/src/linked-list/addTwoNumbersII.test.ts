import { test, expect } from "bun:test";
import { addTwoNumbers } from "./addTwoNumbersII";
import { ListNode } from "./ListNode";

test("addTwoNumbers #1", () => {
	const l1 = new ListNode(7, new ListNode(2, new ListNode(4, new ListNode(3))));
	const l2 = new ListNode(5, new ListNode(6, new ListNode(4)));

	const result = addTwoNumbers(l1, l2);
	expect(result?.val).toBe(7);
	expect(result?.next?.val).toBe(8);
	expect(result?.next?.next?.val).toBe(0);
	expect(result?.next?.next?.next?.val).toBe(7);
});
