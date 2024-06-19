import { ListNode } from "./ListNode.ts";

function addTwoNumbers(l1: ListNode | null, l2: ListNode | null): ListNode | null {
	let a = reverse(l1);
	let b = reverse(l2);

	let head = new ListNode();
	let curr = head;
	let carry = 0;

	while (a || b) {
		let sum = carry + (a?.val ?? 0) + (b?.val ?? 0);
		curr.val = sum % 10;
		carry = sum > 9 ? 1 : 0; // Math.floor(sum / 10) => 12 / 10 = 1.2 => 1

		a = a?.next ?? null;
		b = b?.next ?? null;

		if (a || b) {
			curr.next = new ListNode();
			curr = curr.next;
		}
	}

	if (carry) {
		curr.next = new ListNode(carry);
	}

	return reverse(head);
}

function reverse(head: ListNode | null): ListNode | null {
	let prev = null;
	let curr = head;

	while (curr) {
		const next = curr.next;
		curr.next = prev;
		prev = curr;
		curr = next;
	}

	return prev;
}

export { addTwoNumbers };
