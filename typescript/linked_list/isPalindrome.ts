import type { ListNode } from "./ListNode";

export function isPalindrome(head: ListNode | null): boolean {
	const stack = [];

	let current = head;
	while (current) {
		stack.push(current.val);
		current = current.next;
	}

	let isPalindrome = true;
	current = head;
	while (current) {
		if (current.val !== stack.pop()) {
			isPalindrome = false;
			break;
		}
		current = current.next;
	}

	return isPalindrome;
}
