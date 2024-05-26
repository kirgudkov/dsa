import type { ListNode } from "./ListNode.ts";

export function removeElements(head: ListNode | null, val: number): ListNode | null {
	let prev: ListNode | null = null;
	let current: ListNode | null = head;

	while (current) {
		if (current.val === val) {
			// We've found the match;
			// Now we need to figure out if current is head or not, because it affects the way we move pointers
			if (!prev) { // No prev => current is head;
				head = head?.next ?? null; // We also could use current.next since current is actually the head;
				// prev stays null;
			} else {
				// Drop current;
				prev.next = current.next;
			}
		} else {
			// Move prev only when current didn't match
			prev = current;
		}

		current = current.next ?? null;
	}

	return head;
}
