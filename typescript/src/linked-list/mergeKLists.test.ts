import { test, expect } from "bun:test";
import { mergeKLists } from "./mergeKLists";
import { ListNode } from "./ListNode";

test("mergeKLists #1", () => {
	const l1 = new ListNode(1, new ListNode(4, new ListNode(5)));
	const l2 = new ListNode(1, new ListNode(3, new ListNode(4)));
	const l3 = new ListNode(2, new ListNode(6));

	expect(mergeKLists([l1, l2, l3])).toEqual(
		new ListNode(1,
			new ListNode(1,
				new ListNode(2,
					new ListNode(3,
						new ListNode(4,
							new ListNode(4,
								new ListNode(5,
									new ListNode(6)
								)
							)
						)
					)
				)
			)
		)
	);
});
