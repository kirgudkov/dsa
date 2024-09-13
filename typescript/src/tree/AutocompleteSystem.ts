class AutocompleteSystem {
	private Node = class {
		char: string;
		is_terminal: boolean;
		children: Map<string, this>;

		rank?: number;
		str?: string;

		constructor(char: string) {
			this.char = char;
			this.is_terminal = false;
			this.children = new Map<string, this>();
		}
	};

	private root = new this.Node("");
	private buffer: string[] = [];

	constructor(sentences: string[], times: number[]) {
		sentences.forEach((str, i) =>
			this.insert_str(str, times[i])
		);
	}

	input(char: string): string[] {
		if (char == "#") {
			this.insert_str(this.buffer.join(""));
			return this.buffer = [];
		} else {
			this.buffer.push(char);
			return this.search_buf();
		}
	}

	private insert_str(str: string, rank: number = 1) {
		let node = this.root;

		for (let char of str) {
			if (!node.children.has(char)) {
				node.children.set(char, new this.Node(char));
			}

			node = node.children.get(char)!;
		}

		node.is_terminal = true;
		node.rank = (node.rank || 0) + rank;
		node.str = str;
	}

	private search_buf(): string[] {
		// 1. Find the last node that represents the current buffer
		let node = this.root;

		for (const char of this.buffer) {
			if (!node.children.has(char)) {
				// No words were found with the current prefix
				return [];
			} else {
				node = node.children.get(char)!;
			}
		}

		// 2. Now we have the starting node from which we will collect all the words, sort them and return top 3
		const words: [word: string, heat: number][] = [];
		const queue = [node];

		while (queue.length) {
			const node = queue.shift()!;

			if (node.is_terminal) {
				words.push([node.str!, node.rank!]);
			}

			for (const child of node.children.values()) {
				queue.push(child);
			}
		}

		// Probably we could use a max heap here to avoid sorting
		return words.sort(([a_str, a_rank], [b_str, b_rank]) => {
				if (a_rank == b_rank) {
					return a_str.localeCompare(b_str);
				}

				return b_rank - a_rank;
			})
			.slice(0, 3)
			.map(([str]) => str);
	}
}

export { AutocompleteSystem };
