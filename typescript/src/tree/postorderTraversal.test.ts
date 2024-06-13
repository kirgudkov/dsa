import { describe, test, expect } from "bun:test";
import { postorderTraversal } from "./postorderTraversal";
import { TreeNode } from "./TreeNode";

describe("postorderTraversal", () => {
	test("Example test case", () => {
		const root = new TreeNode(1, new TreeNode(2), new TreeNode(3));
		expect(postorderTraversal(root)).toEqual([2, 3, 1]);
	});

	test("Empty tree", () => {
		expect(postorderTraversal(null)).toEqual([]);
	});

	test("Single node", () => {
		const root = new TreeNode(1);
		expect(postorderTraversal(root)).toEqual([1]);
	});

	test("Left child only", () => {
		const root = new TreeNode(1, new TreeNode(2));
		expect(postorderTraversal(root)).toEqual([2, 1]);
	});

	test("Right child only", () => {
		const root = new TreeNode(1, null, new TreeNode(3));
		expect(postorderTraversal(root)).toEqual([3, 1]);
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

		expect(postorderTraversal(root)).toEqual([4, 5, 2, 6, 7, 3, 1]);
	});
});
