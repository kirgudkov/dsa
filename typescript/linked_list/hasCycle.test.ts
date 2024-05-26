import { describe, test, expect } from "bun:test";
import { hasCycle } from "./hasCycle";
import { ListNode } from "./ListNode.ts";

describe("linked_list/hasCycle", () => {
	test("Example 1", () => {
		const node_3 = new ListNode(3);
		const node_2 = new ListNode(2, node_3);
		const node_1 = new ListNode(1, node_2);
		const head = new ListNode(0, node_1);
		node_3.next = head;

		expect(hasCycle(head)).toBe(true);
	});

	test("Example 2", () => {
		const node_2 = new ListNode(2);
		const head = new ListNode(1, node_2);
		node_2.next = head;

		expect(hasCycle(head)).toBe(true);
	});

	test("Example 3", () => {
		expect(hasCycle(new ListNode(1))).toBe(false);
	});

	test("Example 4", () => {
		const node_2 = new ListNode(2);
		const head = new ListNode(1, node_2);

		expect(hasCycle(head)).toBe(false);
	});
});
