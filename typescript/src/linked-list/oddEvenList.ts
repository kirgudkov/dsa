import type { ListNode } from "./ListNode.ts";

export function oddEvenList(head: ListNode | null): ListNode | null {
	if (!head || !head.next) {
		return head;
	}

	const first_even: ListNode = head.next;

	let last_odd: ListNode = head;
	let current: ListNode | null = head.next;

	while (current?.next) {
		// current is always even;
		// current.next is always odd;

		// We are going to move current.next to the position between last_odd and first_even.
		// current.next will become the new last_odd.
		// current will point to the node after current.next (next even node).

		// Stash next even node, because we're about to mutate the reference:
		const next_even: ListNode | null = current.next.next;

		// Make the current.next the new last_odd:
		current.next.next = first_even;
		last_odd.next = current.next;
		last_odd = current.next;

		// Connect current node with next_even:
		current.next = next_even;

		// Update current pointer
		current = next_even;
	}

	return head;
}
