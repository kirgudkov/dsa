import { TreeNode } from "./TreeNode.ts";

export function serialize(root: TreeNode | null): string {
	return root
		? root.val.toString() + "," + serialize(root?.left) + "," + serialize(root?.right)
		: "#";
}

export function deserialize(data: string): TreeNode | null {
	return parse(data)[0];
}

function parse(str: string): [TreeNode | null, string] {
	let _str = str;

	if (str[0] === ",") {
		_str = str.substring(1);
	}

	if (_str[0] === "#") {
		return [null, _str.substring(1)];
	}

	const comma = _str.indexOf(",");
	const val = _str.substring(0, comma);

	const root = new TreeNode(parseInt(val));

	const [left, str1] = parse(_str.substring(comma + 1));
	root.left = left;

	const [right, str2] = parse(str1);
	root.right = right;

	return [root, str2];
}
