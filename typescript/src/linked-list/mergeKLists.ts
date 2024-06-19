import { ListNode } from "./ListNode.ts";
import { mergeTwoLists } from "./mergeTwoLists.ts";

function mergeKLists(lists: Array<ListNode | null>): ListNode | null {
	if (lists.length === 0) {
		return null;
	}

	let head: ListNode | null = lists[0];

	for (let i = 1; i < lists.length; i++) {
		head = mergeTwoLists(head, lists[i]);
	}

	return head;
}

export { mergeKLists };
