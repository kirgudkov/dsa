import { TreeNode } from "./TreeNode.ts";

// https://leetcode.com/problems/serialize-and-deserialize-bst
// Not super performant but works;
// TC: O(n)
function serialize(root: TreeNode | null): string {
	return root
		? root.val.toString() + "," + serialize(root?.left) + "," + serialize(root?.right)
		: "#";
}

function deserialize(data: string): TreeNode | null {
	return parse(data)[0];
}

function parse(serialized: string): [TreeNode | null, string] {
	let str = serialized;

	if (str[0] === ",") {
		str = str.substring(1);
	}

	if (str[0] === "#") {
		return [null, str.substring(1)];
	}

	const next_comma_index = str.indexOf(",");

	const node = new TreeNode(
		parseInt(str.substring(0, next_comma_index))
	);

	const [left, str1] = parse(str.substring(next_comma_index + 1));
	const [right, str2] = parse(str1);

	node.left = left;
	node.right = right;

	return [node, str2];
}

export { serialize, deserialize };
