import { test, expect } from "bun:test";
import { deleteNode } from "./BSTDeleteNode";
import { TreeNode } from "./TreeNode";

test("BSTDeleteNode", () => {
	const root = new TreeNode(5);
	root.left = new TreeNode(3);
	root.right = new TreeNode(6);
	root.left.left = new TreeNode(2);
	root.left.right = new TreeNode(4);
	root.right.right = new TreeNode(7);

	const newRoot = deleteNode(root, 3);

	expect(newRoot!.val).toBe(5);
	expect(newRoot!.left!.val).toBe(4);
	expect(newRoot!.left!.left!.val).toBe(2);
	expect(newRoot!.left!.right).toBe(null);
	expect(newRoot!.right!.val).toBe(6);
	expect(newRoot!.right!.right!.val).toBe(7);
});

test("BSTDeleteNode", () => {
	const root = new TreeNode(5);
	root.left = new TreeNode(3);
	root.right = new TreeNode(6);
	root.left.left = new TreeNode(2);
	root.left.right = new TreeNode(4);
	root.right.right = new TreeNode(7);

	const newRoot = deleteNode(root, 5);

	expect(newRoot!.val).toBe(6);
	expect(newRoot!.left!.val).toBe(3);
	expect(newRoot!.left!.left!.val).toBe(2);
	expect(newRoot!.left!.right!.val).toBe(4);
	expect(newRoot!.right!.val).toBe(7);
});
