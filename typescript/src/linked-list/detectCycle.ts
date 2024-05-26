import type { ListNode } from "./ListNode.ts";

export function detectCycle(head: ListNode | null): ListNode | null {
	const hash_map = new Set<ListNode>();

	while (head) {
		if (hash_map.has(head)) {
			return head;
		}

		hash_map.add(head);
		head = head.next;
	}

	return null;
}
