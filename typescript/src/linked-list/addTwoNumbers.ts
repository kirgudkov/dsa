import { ListNode } from "./ListNode.ts";

export function addTwoNumbers(l1: ListNode | null, l2: ListNode | null): ListNode | null {
	const head = new ListNode();

	let current = head;
	let carry = 0;

	while (l1 || l2) {
		let sum = (l1?.val ?? 0) + (l2?.val ?? 0) + carry;

		carry = sum / 10 | 0; // Fancy way to do Math.floor(sum / 10); or in this case it could be just sum > 9 ? 1 : 0
		current.val = sum % 10;

		l1 = l1?.next ?? null;
		l2 = l2?.next ?? null;

		if (l1 || l2) {
			current.next = new ListNode();
			current = current.next;
		}
	}

	if (carry) {
		current.next = new ListNode(carry);
	}

	return head;
}
