import { test, expect } from "bun:test";
import { MyCircularQueue } from "./MyCircularQueue.ts";

test("Test 1", () => {
	const queue = new MyCircularQueue(3);

	expect(queue.enQueue(1)).toBe(true);
	expect(queue.enQueue(2)).toBe(true);
	expect(queue.enQueue(3)).toBe(true);
	expect(queue.enQueue(4)).toBe(false);

	expect(queue.Rear()).toBe(3);
	expect(queue.isFull()).toBe(true);

	expect(queue.deQueue()).toBe(true);
	expect(queue.enQueue(4)).toBe(true);
	expect(queue.Rear()).toBe(4);

	expect(queue.deQueue()).toBe(true);
	expect(queue.deQueue()).toBe(true);
	expect(queue.deQueue()).toBe(true);
	expect(queue.deQueue()).toBe(false);
	expect(queue.isEmpty()).toBe(true);
	expect(queue.Front()).toBe(-1);
	expect(queue.Rear()).toBe(-1);

	expect(queue.enQueue(1)).toBe(true);
	expect(queue.Front()).toBe(1);
	expect(queue.Rear()).toBe(1);
});
