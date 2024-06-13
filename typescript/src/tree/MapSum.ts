class MapSum {
	root: TrieNode;

	constructor() {
		this.root = new TrieNode("");
	}

	insert(key: string, val: number): void {
		let node = this.root;

		const _insert_char = (node: TrieNode, char: string): TrieNode => {
			if (node.children.has(char)) {
				return node.children.get(char)!;
			} else {
				const child = new TrieNode(char);
				node.children.set(char, child);
				return child;
			}
		};

		for (const char of key) {
			node = _insert_char(node, char);
		}

		node.terminal = true;
		node.value = val;
	}

	sum(prefix: string): number {
		let node = this.root;
		let sum = 0;

		for (const char of prefix) {
			if (node.children.has(char)) {
				node = node.children.get(char)!;
			} else {
				return 0;
			}
		}

		const traverse = (node: TrieNode) => {
			if (node.terminal) {
				sum += node.value ?? 0;
			}

			for (const [_, child] of node.children) {
				traverse(child);
			}
		};

		traverse(node);

		return sum;
	}
}

class TrieNode {
	char: string;
	children: Map<string, TrieNode>;
	terminal: boolean;
	value?: number;

	constructor(char: string) {
		this.char = char;
		this.children = new Map();
		this.terminal = false;
	}
}

export { MapSum };
