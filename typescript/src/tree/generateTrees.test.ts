import { test, expect } from "bun:test";
import { generateTrees } from "./generateTrees.ts";
import { TreeNode } from "./TreeNode.ts";

test("generateTrees(3)", () => {
	const result = generateTrees(3);

	expect(result).toStrictEqual([
		new TreeNode(1, null, new TreeNode(2, null, new TreeNode(3))),
		new TreeNode(1, null, new TreeNode(3, new TreeNode(2), null)),
		new TreeNode(2, new TreeNode(1), new TreeNode(3)),
		new TreeNode(3, new TreeNode(1, null, new TreeNode(2)), null),
		new TreeNode(3, new TreeNode(2, new TreeNode(1), null), null)
	]);
});
