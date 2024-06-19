import { test, expect } from "bun:test";
import { mergeTwoLists } from "./mergeTwoLists.ts";
import { ListNode } from "./ListNode.ts";

test("mergeTwoLists #1", () => {
	const l1 = new ListNode(1, new ListNode(4, new ListNode(5)));
	const l2 = new ListNode(1, new ListNode(3, new ListNode(4)));

	expect(mergeTwoLists(l1, l2)).toEqual(
		new ListNode(1,
			new ListNode(1,
				new ListNode(3,
					new ListNode(4,
						new ListNode(4,
							new ListNode(5)
						)
					)
				)
			)
		)
	);
});
