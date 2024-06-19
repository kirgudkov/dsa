import { expect, test } from "bun:test";
import { serialize, deserialize } from "./BSTSerializer";
import { TreeNode } from "./TreeNode";

const root = new TreeNode(4);
root.left = new TreeNode(2);
root.right = new TreeNode(6);
root.left.left = new TreeNode(1);
root.left.right = new TreeNode(3);
root.right.left = new TreeNode(5);
root.right.right = new TreeNode(7);

test("serialize", () => {
	expect(serialize(root)).toBe("4,2,1,#,#,3,#,#,6,5,#,#,7,#,#");
});

test("deserialize", () => {
	const deserialized = deserialize(serialize(root));

	expect(deserialized).toEqual(root);
	expect(deserialized!.left).toEqual(root.left);
	expect(deserialized!.right).toEqual(root.right);
	expect(deserialized!.left!.left).toEqual(root.left!.left);
	expect(deserialized!.left!.right).toEqual(root.left!.right);
	expect(deserialized!.right!.left).toEqual(root.right!.left);
	expect(deserialized!.right!.right).toEqual(root.right!.right);
});
