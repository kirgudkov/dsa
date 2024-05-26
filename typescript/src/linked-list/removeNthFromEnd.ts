import type { ListNode } from "./ListNode.ts";

export function removeNthFromEnd(head: ListNode | null, n: number): ListNode | null {
	const arr = [head];

	let current = head;
	while (current && current.next) {
		arr.push(current.next);
		current = current.next;
	}

	// More likely that we got n == list.size; we need to remove head
	if ((arr.length - n - 1) === -1) {
		return arr[1] ?? null; // New head or null if list had one item
	}

	const prev = arr[arr.length - n - 1];

	if (prev) {
		prev.next = arr[arr.length - n + 1] ?? null;
	}

	return head;
}
