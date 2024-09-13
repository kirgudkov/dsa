import type { ListNode } from "./ListNode.ts";

export function removeElements(head: ListNode | null, val: number): ListNode | null {
	let prev: ListNode | null = null;
	let current: ListNode | null = head;

	while (current) {
		if (current.val == val) {
			if (!prev) { // No prev => current is head;
				head = current.next;
			} else {
				prev.next = current.next;
			}
		} else {
			prev = current;
		}

		current = current.next;
	}

	return head;
}
