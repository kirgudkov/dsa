import type { TreeNode } from "./TreeNode.ts";

// LNR (in-order).
class BSTIterator {
	private i: number = 0;
	private list: number[] = [];

	constructor(root: TreeNode | null) {
		if (!root) {
			return;
		}

		let current: TreeNode | null = root;
		const stack: TreeNode[] = [];

		while (current || stack.length) {
			while (current) {
				stack.push(current);
				current = current.left;
			}

			current = stack.pop()!;

			this.list.push(current.val);
			current = current.right;
		}
	}

	next(): number {
		return this.list[this.i++];
	}

	hasNext(): boolean {
		return this.i < this.list.length;
	}
}

export { BSTIterator };
