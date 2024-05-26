import type { ListNode } from "./ListNode.ts";

export function oddEvenList(head: ListNode | null): ListNode | null {
	if (!head) {
		return null;
	}

	// If length is 1, nothing to change;
	if (!head.next) {
		return head;
	}

	let current: ListNode | null = head.next;

	// Border is a first even node. It'll never change. We are going to insert each odd node right before it.
	//                                  p   c  n               p      c  n            p      c     n
	// It will be something like this: [1, |2, 3, 4, 5] => [1, 3, |2, 4, 5] => [1, 3, 5, |2, 4]; | null
	let border: ListNode | null = head.next;
	let border_prev: ListNode | null = head;

	while (current?.next) {
		border_prev.next = current.next;
		border_prev = current.next;

		const next_next: ListNode | null = current.next.next;

		current.next.next = border;
		current.next = next_next;
		current = next_next;
	}

	return head;
}
