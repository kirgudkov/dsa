import { TreeNode } from "./TreeNode.ts";

// https://leetcode.com/problems/serialize-and-deserialize-bst
// Not super performant but works;
// TC: O(n)
function serialize(root: TreeNode | null): string {
	return root
		? root.val.toString() + DELIMITER + serialize(root?.left) + DELIMITER + serialize(root?.right)
		: END;
}

function deserialize(data: string): TreeNode | null {
	return parse(data)[0];
}

function parse(str: string): [TreeNode | null, string] {
	if (str[0] == DELIMITER) {
		str = str.slice(1);
	}

	if (str[0] == END) {
		return [null, str.slice(1)];
	}

	const val_end = str.indexOf(DELIMITER);

	const node = new TreeNode(
		parseInt(str.slice(0, val_end))
	);

	const [left, str1] = parse(str.slice(val_end + 1));
	const [right, str2] = parse(str1);

	node.left = left;
	node.right = right;

	return [node, str2];
}

const DELIMITER = ",";
const END = "#";

export { serialize, deserialize };
