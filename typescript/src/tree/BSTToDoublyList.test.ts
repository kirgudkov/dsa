import { test, expect } from "bun:test";
import { treeToDoublyList } from "./BSTToDoublyList";

test("BSTToDoublyList", () => {
	const root = {
		val: 4,
		left: {
			val: 2,
			left: { val: 1, left: null, right: null },
			right: { val: 3, left: null, right: null }
		},
		right: {
			val: 5,
			left: null,
			right: null
		}
	};
	const head = treeToDoublyList(root);
	expect(head?.val).toBe(1);
	expect(head?.right?.val).toBe(2);
	expect(head?.left?.val).toBe(5);

	expect(head?.right?.right?.val).toBe(3);
	expect(head?.right?.left?.val).toBe(1);

	expect(head?.right?.right?.right?.val).toBe(4);
	expect(head?.right?.right?.left?.val).toBe(2);

	expect(head?.right?.right?.right?.right?.val).toBe(5);
	expect(head?.right?.right?.right?.left?.val).toBe(3);

	expect(head?.right?.right?.right?.right?.right?.val).toBe(1);
	expect(head?.right?.right?.right?.right?.left?.val).toBe(4);
});
