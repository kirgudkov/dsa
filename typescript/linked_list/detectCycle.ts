import type { ListNode } from "./ListNode";

export function detectCycle(head: ListNode | null): ListNode | null {
	if (!head) {
		return null;
	}

	const hash_map = new Map<ListNode, number>([[head, 1]]);

	let current = head;

	while (current?.next) {
		if (hash_map.has(current.next)) {
			return current.next;
		}

		hash_map.set(current.next, 1);
		current = current.next;
	}

	return null;
}
