import type { ListNode } from "./ListNode.ts";

export function reverseList(head: ListNode | null): ListNode | null {
	let prev = null;
	let curr = head;

	while (curr) {
		const next = curr.next;
		curr.next = prev;

		prev = curr;
		curr = next;
	}

	return prev;
}
