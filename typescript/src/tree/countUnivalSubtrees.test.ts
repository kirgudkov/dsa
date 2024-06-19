import { test, expect } from "bun:test";
import { countUnivalSubtrees } from "./countUnivalSubtrees";
import { TreeNode } from "./TreeNode";

test("countUnivalSubtrees #1", () => {
	const root = new TreeNode(5);
	root.left = new TreeNode(1);
	root.right = new TreeNode(5);
	root.left.left = new TreeNode(5);
	root.left.right = new TreeNode(5);
	root.right.right = new TreeNode(5);

	expect(countUnivalSubtrees(root)).toBe(4);
});

test("countUnivalSubtrees #2", () => {
	const root = new TreeNode(1);
	expect(countUnivalSubtrees(root)).toBe(1);
});

test("countUnivalSubtrees #3", () => {
	const root = new TreeNode(5);
	root.left = new TreeNode(5);
	root.right = new TreeNode(5);
	root.left.left = new TreeNode(5);
	root.left.right = new TreeNode(5);
	root.right.right = new TreeNode(5);
	root.right.right.right = new TreeNode(5);

	expect(countUnivalSubtrees(root)).toBe(7);
});

test("countUnivalSubtrees #4", () => {
	const root = new TreeNode(1);
	root.left = new TreeNode(1);
	root.right = new TreeNode(1);
	root.left.left = new TreeNode(5);
	root.left.right = new TreeNode(5);
	root.right.right = new TreeNode(5);

	expect(countUnivalSubtrees(root)).toBe(3);
});
