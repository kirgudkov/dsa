// https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array
// Trie approach. Works fine but gives TLE on Leetcode.
function findMaximumXOR(nums: number[]): number {
	let max_xor = 0;

	const root = new TrieNode(-1);
	const size = Math.max(...nums).toString(2).length;

	const insert = (num: number) => {
		let node = root;

		for (let i = 0; i < size; i++) {
			const bit = (num >> (size - i - 1)) & 1;

			if (node.children[bit]) {
				node = node.children[bit];
			} else {
				const child = new TrieNode(bit);
				node.children[bit] = child;
				node = child;
			}
		}

		node.value = num;
	};

	const find_max_xor = (num: number): number => {
		let node = root;

		for (let i = 0; i < size; i++) {
			const bit = (num >> (size - i - 1)) & 1;

			if (node.children[1 - bit]) {
				node = node.children[1 - bit];
			} else {
				node = node.children[bit]!;
			}
		}

		return node.value ?? 0;
	};

	insert(nums[0]);

	for (let i = 1; i < nums.length; i++) {
		max_xor = Math.max(max_xor, nums[i] ^ find_max_xor(nums[i]));
		insert(nums[i]);
	}

	return max_xor;
}

class TrieNode {
	bit: number;
	children: {
		[bit: number]: TrieNode;
	};
	value?: number;

	constructor(bit: number) {
		this.bit = bit;
		this.children = {};
	}
}

export { findMaximumXOR };
