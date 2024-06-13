import { test, expect } from "bun:test";
import { WordDictionary } from "./WordDictionary";

test("WordDictionary", () => {
	const wordDictionary = new WordDictionary();
	wordDictionary.addWord("bad");
	wordDictionary.addWord("dad");
	wordDictionary.addWord("mad");
	expect(wordDictionary.search("pad")).toBe(false);
	expect(wordDictionary.search("bad")).toBe(true);
	expect(wordDictionary.search(".ad")).toBe(true);
	expect(wordDictionary.search("b..")).toBe(true);
	expect(wordDictionary.search("..d")).toBe(true);
	expect(wordDictionary.search("..a")).toBe(false);
	expect(wordDictionary.search(".a.")).toBe(true);
	expect(wordDictionary.search(".a")).toBe(false);
});
