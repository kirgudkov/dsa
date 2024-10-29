import { test, expect } from "bun:test";
import { LRUCache } from "./LRUCache";

test("LRUCache #1", () => {
	const cache = new LRUCache<number, number>(2);

	cache.put(1, 1);
	cache.put(2, 2);
	expect(cache.get(1)).toBe(1);
	cache.put(3, 3);
	expect(cache.get(2)).toBe(-1);
	cache.put(4, 4);
	expect(cache.get(1)).toBe(-1);
	expect(cache.get(3)).toBe(3);
	expect(cache.get(4)).toBe(4);
	cache.put(5, 5);
	expect(cache.get(3)).toBe(-1);
	expect(cache.get(4)).toBe(4);
	cache.put(6, 6);
	expect(cache.get(4)).toBe(4);
	expect(cache.get(5)).toBe(-1);
});

test("LRUCache #2", () => {
	const cache = new LRUCache<number, number>(1);

	cache.put(2, 1);
	expect(cache.get(2)).toBe(1);
	cache.put(3, 2);
	expect(cache.get(2)).toBe(-1);
	expect(cache.get(3)).toBe(2);
});

test("LRUCache #3", () => {
	const cache = new LRUCache<number, number>(2);

	cache.put(2, 1);
	cache.put(3, 2);
	expect(cache.get(3)).toBe(2);
	expect(cache.get(2)).toBe(1);
	cache.put(4, 3);
	expect(cache.get(2)).toBe(1);
	expect(cache.get(3)).toBe(-1);
	expect(cache.get(4)).toBe(3);
});
