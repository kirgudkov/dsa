import { describe, test, expect } from "bun:test";
import { isPalindrome } from "./isPalindrome";

describe("isPalindrome", () => {
	test("Example 1", () => {
		const head = {
			val: 1,
			next: {
				val: 2,
				next: {
					val: 2,
					next: {
						val: 1,
						next: null
					}
				}
			}
		};

		expect(isPalindrome(head)).toBe(true);
	});

	test("Example 2", () => {
		const head = {
			val: 1,
			next: {
				val: 2,
				next: null
			}
		};

		expect(isPalindrome(head)).toBe(false);
	});
});
