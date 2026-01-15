import { useMemo } from "react";
import { FuzzySearchOptions, FuzzySearchResult, fuzzySearch } from "@utils/fuzzySearch";

export function useFuzzySearch<T>(items: T[], query: string, options: FuzzySearchOptions = {}): T[] {
  const { keys, multiToken, caseSensitive, sortByScore, tokenSplitter } = options;
  const normalizedQuery = query?.trim() ?? "";

  return useMemo(() => {
    if (normalizedQuery.length === 0) return items;
    const results = fuzzySearch(items, normalizedQuery, {
      keys,
      multiToken,
      caseSensitive,
      sortByScore,
      tokenSplitter,
      includeMatches: false,
    });
    return results.map((result) => result.item);
  }, [items, normalizedQuery, keys, multiToken, caseSensitive, sortByScore, tokenSplitter]);
}

export function useFuzzySearchWithScore<T>(items: T[], query: string, options: FuzzySearchOptions = {}): FuzzySearchResult<T>[] {
  const { keys, multiToken, caseSensitive, sortByScore, tokenSplitter, includeMatches } = options;

  return useMemo(
    () => fuzzySearch(items, query, { keys, multiToken, caseSensitive, sortByScore, tokenSplitter, includeMatches }),
    [items, query, keys, multiToken, caseSensitive, sortByScore, tokenSplitter, includeMatches]
  );
}
