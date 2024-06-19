import { ListNode } from "./ListNode.ts";

function insert(head: ListNode | null, insertVal: number): ListNode | null {
	if (!head) {
		const node = new ListNode(insertVal);
		node.next = node;

		return node;
	}

	let node = head;

	while (node.next && node.next !== head) {
		if (node.val <= node.next.val) {
			if (insertVal >= node.val && insertVal <= node.next.val) {
				break;
			}
		} else {
			if (insertVal >= node.val || insertVal <= node.next.val) {
				break;
			}
		}

		node = node.next;
	}

	const next = node.next;
	node.next = new ListNode(insertVal, next);

	return head;
}

export { insert };
