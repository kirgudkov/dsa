import type { ListNode } from "./ListNode.ts";

export function isPalindrome(head: ListNode | null): boolean {
	const stack = [];
	let current = head;

	while (current) {
		stack.push(current.val);
		current = current.next;
	}

	current = head;

	while (current) {
		if (current.val != stack.pop()) {
			return false;
		}

		current = current.next;
	}

	return true;
}
