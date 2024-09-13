import type { ListNode } from "./ListNode.ts";

export function oddEvenList(head: ListNode | null): ListNode | null {
	if (!head || !head.next) {
		return head;
	}

	let last_odd: ListNode = head;
	let current_even: ListNode | null = head.next;

	const first_even: ListNode = head.next;

	/**
	 * The idea is to take the node following current_even (which is always odd) and put it in betweeen last_odd and first_even.
	 * Moved node becomes new last_odd.
	 */
	while (current_even?.next) {
		const next_even: ListNode | null = current_even.next.next;

		current_even.next.next = first_even;
		last_odd.next = current_even.next;
		last_odd = current_even.next;
		current_even.next = next_even;
		current_even = next_even;
	}

	return head;
}
