import { ListNode } from "./ListNode.ts";

function insert(head: ListNode | null, insertVal: number): ListNode | null {
	if (!head) {
		return new ListNode(insertVal);
	}

	let min = head;
	let max = head;

	let curr = head.next;
	while (curr && curr !== head) {
		if (curr.val <= min.val) {
			min = curr;
		}

		if (curr.val >= max.val) {
			max = curr;
		}

		curr = curr.next;
	}

	if (min.val === max.val || insertVal > max.val || insertVal < min.val) {
		max.next = new ListNode(insertVal, min);
		return head;
	}

	let prev = min;
	let next = min.next;

	while (next && next !== min) {
		if (next?.val >= insertVal && insertVal >= prev.val) {
			prev.next = new ListNode(insertVal, next);
			return head;
		}

		prev = next;
		next = next.next;
	}

	return head;
}

export { insert };
