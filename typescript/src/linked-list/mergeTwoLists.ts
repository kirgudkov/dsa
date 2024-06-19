import { ListNode } from "./ListNode.ts";

function mergeTwoLists(a: ListNode | null, b: ListNode | null): ListNode | null {
	let dummy = new ListNode();
	let current = dummy;

	while (a && b) {
		if (a.val < b.val) {
			current.next = a;
			a = a.next;
		} else {
			current.next = b;
			b = b.next;
		}

		current = current.next;
	}

	current.next = a || b;

	return dummy.next;
}

export { mergeTwoLists };
