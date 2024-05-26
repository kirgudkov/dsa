import { describe, test, expect } from "bun:test";
import { ListNode } from "./ListNode.ts";
import { oddEvenList } from "./oddEvenList.ts";

describe("src/linked-list/oddEvenList", () => {
	test("Example 1", () => {
		const head: ListNode =
			new ListNode(1,
				new ListNode(2,
					new ListNode(3,
						new ListNode(4,
							new ListNode(5)
						)
					)
				)
			);

		const result = oddEvenList(head);

		expect(result).toEqual(
			new ListNode(1,
				new ListNode(3,
					new ListNode(5,
						new ListNode(2,
							new ListNode(4)
						)
					)
				)
			)
		);
	});

	test("Example 2", () => {
		const head: ListNode =
			new ListNode(2,
				new ListNode(1,
					new ListNode(3,
						new ListNode(5,
							new ListNode(6,
								new ListNode(4,
									new ListNode(7)
								)
							)
						)
					)
				)
			);

		const result = oddEvenList(head);

		expect(result).toEqual(
			new ListNode(2,
				new ListNode(3,
					new ListNode(6,
						new ListNode(7,
							new ListNode(1,
								new ListNode(5,
									new ListNode(4)
								)
							)
						)
					)
				)
			)
		);
	});
});
