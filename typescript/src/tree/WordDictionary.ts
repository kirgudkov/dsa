class WordDictionary {
	root = new TrieNode("");

	addWord(word: string): void {
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
		function search(node: TrieNode, word: string) {
			let curr = node;

			for (let i = 0; i < word.length; i++) {
				if (word[i] == ".") {
					for (const [_, node] of curr.children) {
						if (search(node, word.slice(i + 1))) {
							return true;
						}
					}

					return false;
				}

				if (curr.children.has(word[i])) {
					curr = curr.children.get(word[i])!;
				} else {
					return false;
				}
			}

			return curr.terminal;
		}

		return search(this.root, word);
	}
}

class TrieNode {
	char: string;
	terminal: boolean;
	children: Map<string, TrieNode> = new Map();

	constructor(char: string, terminal: boolean = false) {
		this.char = char;
		this.terminal = terminal;
	}
}

export { WordDictionary };
