import { test, expect } from "bun:test";
import { BSTIterator } from "./BSTIterator";
import { TreeNode } from "./TreeNode";

test("BSTIterator", () => {
	const root = new TreeNode(7);
	root.left = new TreeNode(3);
	root.right = new TreeNode(15);
	root.right.left = new TreeNode(9);
	root.right.right = new TreeNode(20);

	const iterator = new BSTIterator(root);
	expect(iterator.next()).toBe(3);
	expect(iterator.next()).toBe(7);
	expect(iterator.hasNext()).toBe(true);
	expect(iterator.next()).toBe(9);
	expect(iterator.hasNext()).toBe(true);
	expect(iterator.next()).toBe(15);
	expect(iterator.hasNext()).toBe(true);
	expect(iterator.next()).toBe(20);
	expect(iterator.hasNext()).toBe(false);
});
