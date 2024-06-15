class Trie {
	root: TrieNode;

	constructor() {
		this.root = new TrieNode("");
	}

	insert(word: string): void {
		if (word.length == 0 || this.search(word)) {
			return;
		}

		const _insert = (node: TrieNode, char: string): TrieNode => {
			if (!node.children.has(char)) {
				node.children.set(char, new TrieNode(char));
			}

			return node.children.get(char);
		};

		let node = this.root;

		for (const char of word) {
			node = _insert(node, char);
		}

		node.terminal = true;
	}

	search(word: string): boolean {
		let node = this.root;

		for (const char of word) {
			if (!node.children.has(char)) {
				return false;
			}

			node = node.children.get(char);
		}

		return node.terminal;
	}

	startsWith(prefix: string): boolean {
		let node = this.root;

		for (const char of prefix) {
			if (!node.children.has(char)) {
				return false;
			}

			node = node.children.get(char);
		}

		return true;
	}
}

class TrieNode {
	char: string;
	terminal: boolean;
	children = new Map();

	constructor(char: string, terminal: boolean = false) {
		this.char = char;
		this.terminal = terminal;
	}
}

export { Trie };
