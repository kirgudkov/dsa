import { test, expect } from "bun:test";
import { ArrayCircularQueue, LinkedListCircularQueue } from "./MyCircularQueue.ts";

test("Test 1", () => {
	const a_queue = new ArrayCircularQueue(3);
	const l_queue = new LinkedListCircularQueue(3);

	expect(a_queue.enQueue(1)).toBe(true);
	expect(l_queue.enQueue(1)).toBe(true);
	expect(a_queue.enQueue(2)).toBe(true);
	expect(l_queue.enQueue(2)).toBe(true);
	expect(a_queue.enQueue(3)).toBe(true);
	expect(l_queue.enQueue(3)).toBe(true);
	expect(a_queue.enQueue(4)).toBe(false);
	expect(l_queue.enQueue(4)).toBe(false);

	expect(a_queue.Rear()).toBe(3);
	expect(l_queue.Rear()).toBe(3);
	expect(a_queue.isFull()).toBe(true);
	expect(l_queue.isFull()).toBe(true);

	expect(a_queue.deQueue()).toBe(true);
	expect(l_queue.deQueue()).toBe(true);
	expect(a_queue.enQueue(4)).toBe(true);
	expect(l_queue.enQueue(4)).toBe(true);
	expect(a_queue.Rear()).toBe(4);
	expect(l_queue.Rear()).toBe(4);

	expect(a_queue.deQueue()).toBe(true);
	expect(l_queue.deQueue()).toBe(true);
	expect(a_queue.deQueue()).toBe(true);
	expect(l_queue.deQueue()).toBe(true);
	expect(a_queue.deQueue()).toBe(true);
	expect(l_queue.deQueue()).toBe(true);
	expect(a_queue.deQueue()).toBe(false);
	expect(l_queue.deQueue()).toBe(false);
	expect(a_queue.isEmpty()).toBe(true);
	expect(l_queue.isEmpty()).toBe(true);
	expect(a_queue.Front()).toBe(-1);
	expect(l_queue.Front()).toBe(-1);
	expect(a_queue.Rear()).toBe(-1);
	expect(l_queue.Rear()).toBe(-1);

	expect(a_queue.enQueue(1)).toBe(true);
	expect(l_queue.enQueue(1)).toBe(true);
	expect(a_queue.Front()).toBe(1);
	expect(l_queue.Front()).toBe(1);
	expect(a_queue.Rear()).toBe(1);
	expect(l_queue.Rear()).toBe(1);
});
