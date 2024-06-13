import { test, expect } from "bun:test";
import { maxDepth } from "./maxDepth.ts";
import { TreeNode } from "./TreeNode.ts";

test("Large tree", () => {
	const root = new TreeNode(3);
	root.left = new TreeNode(9);
	root.right = new TreeNode(20);
	root.right.left = new TreeNode(15);
	root.right.right = new TreeNode(7);

	expect(maxDepth(root)).toBe(3);
});

test("Two nodes tree", () => {
	const root = new TreeNode(1);
	root.right = new TreeNode(2);

	expect(maxDepth(root)).toBe(2);
});

test("Zero nodes tree", () => {
	const root = null;

	expect(maxDepth(root)).toBe(0);
});
