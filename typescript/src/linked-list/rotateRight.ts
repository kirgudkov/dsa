import type { ListNode } from "./ListNode.ts";

export function rotateRight(head: ListNode | null, k: number): ListNode | null {
	if (!head || !head.next || k === 0) {
		return head;
	}

	let last = head;
	let n = 1;

	while (last.next) {
		last = last.next;
		n++;
	}

	const distance = k % n;

	if (distance === 0) {
		return head;
	}

	let prev = head;
	let new_head;

	let i = 1;
	while (i < n - distance && prev.next) {
		prev = prev.next;
		i++;
	}

	new_head = prev.next;

	prev.next = null;
	last.next = head;

	return new_head;
}
