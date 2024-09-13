import type { ListNode } from "./ListNode.ts";

export function rotateRight(head: ListNode | null, k: number): ListNode | null {
	if (!head || !head.next || k == 0) {
		return head;
	}

	// 1. Count list length
	let len = 2; // 2 is min len because of the prev check
	let tail = head.next;

	while (tail.next) {
		tail = tail.next;
		len++;
	}

	// 2. Calculate new head offset
	let offset = k % len;

	if (offset == 0) {
		return head;
	}

	// 3. Find new tail
	let new_tail = head;

	for (let i = len - offset; i > 1; i--) {
		new_tail = new_tail.next!;
	}

	// 4. Connect old tail and old head, they're just regular contiguous nodes now
	tail.next = head;

	// 5. Break connection between new tail and new head
	const new_head = new_tail.next;
	new_tail.next = null;

	return new_head;
}
