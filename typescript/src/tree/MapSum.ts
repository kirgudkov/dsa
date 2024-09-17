class MapSum {
	root: TrieNode;

	constructor() {
		this.root = new TrieNode("");
	}

	insert(key: string, val: number): void {
		let node = this.root;

		for (const char of key) {
			if (!node.children.has(char)) {
				node.children.set(char, new TrieNode(char));
			}

			node = node.children.get(char)!;
		}

		node.value = val;
	}

	sum(prefix: string): number {
		let node = this.root;

		for (const char of prefix) {
			if (!node.children.has(char)) {
				return 0; // prefix not found;
			}

			node = node.children.get(char)!;
		}

		const count = (node: TrieNode, acc = 0): number => {
			let sum = acc;

			if (node.value != undefined) {
				sum += node.value;
			}

			for (const [_, child] of node.children) {
				sum += count(child, acc);
			}

			return sum;
		};

		return count(node);
	}
}

class TrieNode {
	char: string;
	children: Map<string, TrieNode>;
	value?: number;

	constructor(char: string) {
		this.char = char;
		this.children = new Map();
	}
}

export { MapSum };
