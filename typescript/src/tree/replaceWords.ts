function replaceWords(dictionary: string[], sentence: string): string {
	const trie = new Trie(dictionary);

	return sentence
		.split(" ")
		.map(word => trie.try_word(word) ?? word)
		.join(" ");
}

class Trie {
	root: TrieNode;

	constructor(dictionary: string[]) {
		this.root = new TrieNode("");
		dictionary.forEach(this.insert_word);
	}

	try_word(word: string) {
		let root: string | null = null;
		let node = this.root;

		for (const char of word) {
			if (node.children.has(char)) {
				node = node.children.get(char)!;

				root = root
					? root + node.char
					: node.char;

				if (node.terminal) {
					break;
				}
			} else {
				return null;
			}
		}

		return node.terminal ? root : null;
	};

	private insert_word = (word: string) => {
		let node = this.root;

		for (const char of word) {
			if (!node.children.has(char)) {
				node.children.set(char, new TrieNode(char));
			}

			node = node.children.get(char);
		}

		node.terminal = true;
	};
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

export { replaceWords };
