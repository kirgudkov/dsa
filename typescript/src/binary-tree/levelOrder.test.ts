import { describe, test, expect } from "bun:test";
import { levelOrder } from "./levelOrder.ts";
import { TreeNode } from "./TreeNode.ts";

describe("src/binary-tree/levelOrder", () => {
	test("Example 1", () => {
		const root = new TreeNode(3, new TreeNode(9), new TreeNode(20, new TreeNode(15), new TreeNode(7)));
		expect(levelOrder(root)).toStrictEqual([[3], [9, 20], [15, 7]]);
	});

	test("Example 2", () => {
		const root = new TreeNode(1);
		expect(levelOrder(root)).toStrictEqual([[1]]);
	});

	test("Example 3", () => {
		expect(levelOrder(null)).toStrictEqual([]);
	});
});
