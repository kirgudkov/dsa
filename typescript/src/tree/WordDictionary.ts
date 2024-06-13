class WordDictionary {
	root = new TrieNode("");

	addWord(word: string): void {
		let node = this.root;

		for (const char of word) {
			if (node.children.has(char)) {
				node = node.children.get(char)!;
			} else {
				const child = new TrieNode(char);
				node.children.set(char, child);
				node = child;
			}
		}

		node.terminal = true;
	}

	search(word: string): boolean {
		const _search = (node: TrieNode, word: string): boolean => {
			let curr = node;

			for (let i = 0; i < word.length; i++) {
				if (curr.children.has(word[i])) {
					curr = curr.children.get(word[i])!;
				} else if (word[i] === ".") {
					for (const child of curr.children) {
						if (_search(child[1], word.slice(i + 1))) {
							return true;
						}
					}

					return false;
				} else {
					return false;
				}
			}

			return curr.terminal;
		};

		return _search(this.root, word);
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
