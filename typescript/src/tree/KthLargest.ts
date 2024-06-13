import { TreeNode } from "./TreeNode.ts";

class KthLargest {
	private size: number = 0;
	private readonly k: number;
	private root: TreeNode | null = null;

	constructor(k: number, nums: number[]) {
		this.k = k;

		nums.forEach((num) => {
			this.add(num);
		});
	}

	add(val: number): number {
		this.root = this.insert(this.root, val);
		this.size++;

		if (this.size > this.k) {
			this.root = this.delete(this.root, this.min(this.root));
			this.size--;
		}

		return this.min(this.root);
	}

	private delete(node: TreeNode | null, val: number): TreeNode | null {
		if (!node) {
			return null;
		}

		if (val < node.val) {
			node.left = this.delete(node.left, val);
		} else if (val > node.val) {
			node.right = this.delete(node.right, val);
		} else {
			if (!node.left) {
				return node.right;
			}

			if (!node.right) {
				return node.left;
			}

			node.val = this.min(node.right);
			node.right = this.delete(node.right, node.val);
		}

		return node;
	}

	private min(root: TreeNode | null): number {
		let current = root;

		while (current?.left) {
			current = current.left;
		}

		return current!.val;
	}

	private insert(node: TreeNode | null, val: number): TreeNode {
		if (!node) {
			return new TreeNode(val);
		}

		if (val < node.val) {
			node.left = this.insert(node.left, val);
		} else {
			node.right = this.insert(node.right, val);
		}

		return node;
	};
}

export { KthLargest };
