// Multi-token search utility
// Simple partial-token matching across any token in a field

export interface FuzzySearchOptions {
  // Fields to search in. If empty, searches the entire item as string
  keys?: string[];

  // Enable multi-token search (all tokens must match)
  // Default: true
  multiToken?: boolean;

  // Case sensitive search
  // Default: false
  caseSensitive?: boolean;

  // Sort results by relevance score
  // Default: true
  sortByScore?: boolean;

  // Splitter for tokens
  // Default: /[\s\-_]+/
  tokenSplitter?: RegExp;

  // Include match indices in results
  // Default: false
  includeMatches?: boolean;
}

export interface FuzzySearchResult<T> {
  item: T;
  score: number;
  matches: {
    key: string;
    value: string;
    indices: number[][];
  }[];
}

const DEFAULT_TOKEN_SPLITTER = /[\s\-_]+/;

const normalizeInput = (value: string, caseSensitive: boolean): string => (caseSensitive ? value : value.toLowerCase());

const tokenize = (value: string, splitter: RegExp): string[] => {
  if (!value) return [];
  return value.split(splitter).filter(Boolean);
};

// Find all indices where pattern matches in text
function findMatchIndices(pattern: string, text: string, caseSensitive: boolean): number[][] {
  const indices: number[][] = [];
  const patternValue = caseSensitive ? pattern : pattern.toLowerCase();
  const textValue = caseSensitive ? text : text.toLowerCase();

  let index = textValue.indexOf(patternValue);
  while (index !== -1) {
    indices.push([index, index + pattern.length - 1]);
    index = textValue.indexOf(patternValue, index + 1);
  }

  return indices;
}

// Get nested property value from object
function getNestedValue(obj: any, path: string): string {
  const keys = path.split(".");
  let value = obj;

  for (const key of keys) {
    if (value === null || value === undefined) return "";
    value = value[key];
  }

  return String(value ?? "");
}

function getSearchableTexts<T>(item: T, keys: string[]): string[] {
  if (keys.length > 0) return keys.map((key) => getNestedValue(item, key));
  return [String(item ?? "")];
}

function scoreToken(token: string, candidates: string[]): number {
  let best = 0;

  for (const candidate of candidates) {
    if (!candidate) continue;

    if (candidate === token) return 1;
    const index = candidate.indexOf(token);
    if (index === -1) continue;

    let score = token.length / candidate.length;
    if (index === 0) score = Math.min(1, score + 0.1);
    if (score > best) best = score;
  }

  return best;
}

function scoreTokens(searchTokens: string[], text: string, textTokens: string[]): { match: boolean; score: number } {
  if (searchTokens.length === 0) return { match: false, score: 0 };

  const candidates = textTokens.length > 0 ? [...textTokens, text] : [text];
  let total = 0;

  for (const token of searchTokens) {
    const best = scoreToken(token, candidates);
    if (best === 0) return { match: false, score: 0 };
    total += best;
  }

  return { match: true, score: total / searchTokens.length };
}

function calculateScore<T>(
  item: T,
  normalizedPattern: string,
  normalizedTokens: string[],
  keys: string[],
  options: FuzzySearchOptions
): number {
  const {
    multiToken = true,
    caseSensitive = false,
    tokenSplitter = DEFAULT_TOKEN_SPLITTER,
  } = options;

  let maxScore = 0;
  const searchableTexts = getSearchableTexts(item, keys);

  for (const text of searchableTexts) {
    const searchText = normalizeInput(text, caseSensitive);
    if (!searchText) continue;

    if (searchText === normalizedPattern) return 1;

    const textTokens = tokenize(searchText, tokenSplitter);
    const tokenScore = scoreTokens(multiToken ? normalizedTokens : [normalizedPattern], searchText, textTokens);
    if (tokenScore.match) {
      maxScore = Math.max(maxScore, tokenScore.score);
    }
  }

  return maxScore;
}

// Main multi-token search function :D
export function fuzzySearch<T>(items: T[], pattern: string, options: FuzzySearchOptions = {}): FuzzySearchResult<T>[] {
  const {
    keys = [],
    multiToken = true,
    sortByScore = true,
    caseSensitive = false,
    tokenSplitter = DEFAULT_TOKEN_SPLITTER,
    includeMatches = false,
  } = options;

  const rawPattern = pattern?.trim() ?? "";
  if (rawPattern.length === 0) {
    return items.map((item) => ({
      item,
      score: 1,
      matches: [],
    }));
  }

  const normalizedPattern = normalizeInput(rawPattern, caseSensitive);
  const rawTokens = multiToken ? tokenize(rawPattern, tokenSplitter) : [rawPattern];
  const normalizedTokens = multiToken ? tokenize(normalizedPattern, tokenSplitter) : [normalizedPattern];

  const results: FuzzySearchResult<T>[] = [];

  for (const item of items) {
    const score = calculateScore(item, normalizedPattern, normalizedTokens, keys, {
      ...options,
      caseSensitive,
      tokenSplitter,
    });

    if (score > 0) {
      const matches: FuzzySearchResult<T>["matches"] = [];
      if (includeMatches) {
        const searchableKeys = keys.length > 0 ? keys : [""];
        const matchTokens = multiToken && rawTokens.length > 1 ? rawTokens : [rawPattern];

        for (const key of searchableKeys) {
          const text = key ? getNestedValue(item, key) : String(item ?? "");
          const indices = matchTokens.flatMap((token) => findMatchIndices(token, text, caseSensitive));

          if (indices.length > 0) {
            matches.push({
              key,
              value: text,
              indices,
            });
          }
        }
      }

      results.push({
        item,
        score,
        matches,
      });
    }
  }

  if (sortByScore) {
    results.sort((a, b) => b.score - a.score);
  }

  return results;
}

// Simple multi-token search (backward compatible)
export function multiTokenSearch<T>(items: T[], pattern: string, keys: string[] = []): T[] {
  const results = fuzzySearch(items, pattern, {
    keys,
    multiToken: true,
    sortByScore: false,
  });

  return results.map((r) => r.item);
}
