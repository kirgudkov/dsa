class Trie {
	root: TrieNode;

	constructor() {
		this.root = new TrieNode("");
	}

	insert(word: string): void {
		if (word.length == 0 || this.search(word)) {
			return;
		}

		let node = this.root;

		for (const char of word) {
			if (!node.children.has(char)) {
				node.children.set(char, new TrieNode(char));
			}

			node = node.children.get(char)!;
		}

		node.terminal = true;
	}

	search(word: string): boolean {
		return this.searchPrefixNode(word)?.terminal ?? false;
	}

	startsWith(prefix: string): boolean {
		return this.searchPrefixNode(prefix) != undefined;
	}

	private searchPrefixNode(prefix: string): TrieNode | undefined {
		let node = this.root;

		for (const char of prefix) {
			if (!node.children.has(char)) {
				return;
			}

			node = node.children.get(char)!;
		}

		return node;
	}
}

class TrieNode {
	char: string;
	terminal: boolean;
	children = new Map<string, TrieNode>();

	constructor(char: string, terminal: boolean = false) {
		this.char = char;
		this.terminal = terminal;
	}
}

export { Trie };
