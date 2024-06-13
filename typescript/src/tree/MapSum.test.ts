import { test, expect } from "bun:test";
import { MapSum } from "./MapSum";

test("MapSum", () => {
	const mapSum = new MapSum();
	mapSum.insert("apple", 3);
	expect(mapSum.sum("ap")).toBe(3);
	mapSum.insert("app", 2);
	expect(mapSum.sum("ap")).toBe(5);
});
