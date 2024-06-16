import { test, expect } from "bun:test";
import { inorderSuccessor } from "./inorderSuccessor";
import { TreeNode } from "./TreeNode";

test("inorder successor #1", () => {
	const root = new TreeNode(2);
	root.left = new TreeNode(1);
	root.right = new TreeNode(3);
	expect(inorderSuccessor(root, root.left)).toBe(root);
});

test("inorder successor #2", () => {
	const root = new TreeNode(5);
	root.left = new TreeNode(3);
	root.left.left = new TreeNode(2);
	root.left.left.left = new TreeNode(1);
	root.left.right = new TreeNode(4);
	root.right = new TreeNode(6);
	expect(inorderSuccessor(root, root.left.right)).toBe(root);
	expect(inorderSuccessor(root, root.right)).toBe(null);
});

test("inorder successor #3", () => {
	const root = new TreeNode(2);
	root.right = new TreeNode(3);
	expect(inorderSuccessor(root, root)).toBe(root.right);
});
