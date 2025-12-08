import { Text, TextProps } from "@mantine/core";

export type TokenHandler = {
  pattern: RegExp; // What triggers the token
  render: (match: RegExpMatchArray) => React.ReactNode; // How to render it
};

export interface LocalizedDynamicMessageProps {
  message: string;
  tokens: TokenHandler[];
  textProps?: TextProps & { [key: string]: any };
}
export function LocalizedDynamicMessage({ message, tokens, textProps }: LocalizedDynamicMessageProps) {
  let cursor = 0;
  const parts: React.ReactNode[] = [];

  while (cursor < message.length) {
    let earliestMatch: {
      tokenIndex: number;
      match: RegExpMatchArray;
      start: number;
    } | null = null;

    // Find first matching token ahead of cursor
    for (let i = 0; i < tokens.length; i++) {
      const pattern = new RegExp(tokens[i].pattern, "g");
      pattern.lastIndex = cursor;

      const m = pattern.exec(message);
      if (m && (earliestMatch === null || m.index < earliestMatch.start)) {
        earliestMatch = { tokenIndex: i, match: m, start: m.index };
      }
    }

    // No tokens found → push rest of text
    if (!earliestMatch) {
      parts.push(message.slice(cursor));
      break;
    }

    const { tokenIndex, match, start } = earliestMatch;

    // Push text before token
    if (start > cursor) {
      parts.push(message.slice(cursor, start));
    }

    // Push rendered token
    parts.push(tokens[tokenIndex].render(match));

    // Move cursor past this token
    cursor = start + match[0].length;
  }

  return (
    <Text {...textProps} component="div" style={{ display: "flex", flexWrap: "wrap", alignItems: "center" }}>
      {parts.map((p, i) => (
        <span key={i} style={{ whiteSpace: "pre-wrap" }}>
          {p}
        </span>
      ))}
    </Text>
  );
}
