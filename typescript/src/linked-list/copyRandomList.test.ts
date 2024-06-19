import { test, expect } from "bun:test";
import { copyRandomList, _Node } from "./copyRandomList";

test("copyRandomList #1", () => {
	let root: _Node | null = new _Node(7);
	const node2 = new _Node(13);
	root.next = node2;
	root.random = null;
	const node3 = new _Node(11);
	node2.next = node3;
	node2.random = root;
	const node4 = new _Node(10);
	const node5 = new _Node(1);
	node3.next = node4;
	node3.random = node5;
	node4.next = node5;
	node4.random = node3;
	node5.random = root;

	let result: _Node | null = copyRandomList(root);

	while (root && result) {
		expect(root.val).toBe(result.val);
		expect(root).not.toBe(result);

		if (root.random && result.random) {
			expect(root.random).not.toBe(result.random);
			expect(root.random.val).toBe(result.random.val);
		}

		root = root.next;
		result = result.next;
	}
});
