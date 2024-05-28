import { describe, test, expect } from "bun:test";
import { inorderTraversal } from "./inorderTraversal";
import { TreeNode } from "./TreeNode";

describe("inorderTraversal", () => {
	test("Example test case", () => {
		const root = new TreeNode(1, new TreeNode(2), new TreeNode(3));
		expect(inorderTraversal(root)).toEqual([2, 1, 3]);
	});

	test("Empty tree", () => {
		expect(inorderTraversal(null)).toEqual([]);
	});

	test("Single node", () => {
		const root = new TreeNode(1);
		expect(inorderTraversal(root)).toEqual([1]);
	});

	test("Left child only", () => {
		const root = new TreeNode(1, new TreeNode(2));
		expect(inorderTraversal(root)).toEqual([2, 1]);
	});

	test("Right child only", () => {
		const root = new TreeNode(1, null, new TreeNode(3));
		expect(inorderTraversal(root)).toEqual([1, 3]);
	});

	test("Big tree", () => {
		const root = new TreeNode(1,
			new TreeNode(2,
				new TreeNode(4),
				new TreeNode(5)
			),
			new TreeNode(3,
				new TreeNode(6),
				new TreeNode(7)
			)
		);

		expect(inorderTraversal(root)).toEqual([4, 2, 5, 1, 6, 3, 7]);
	});
});
