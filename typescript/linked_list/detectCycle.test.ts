import { describe, test, expect } from "bun:test";
import { detectCycle } from "./detectCycle";
import { ListNode } from "./ListNode";

describe("detectCycle", () => {
	test("Example 1", () => {
		const node_3 = new ListNode(3, null);
		const node_2 = new ListNode(2, node_3);
		const node_1 = new ListNode(1, node_2);
		const head = new ListNode(0, node_1);
		node_3.next = node_1;

		expect(detectCycle(head)).toBe(node_1);
	});

	test("Example 2", () => {
		const node_2 = new ListNode(2);
		const head = new ListNode(1, node_2);
		node_2.next = head;

		expect(detectCycle(head)).toBe(head);
	});

	test("Example 3", () => {
		expect(detectCycle(new ListNode(1))).toBe(null);
	});
});
