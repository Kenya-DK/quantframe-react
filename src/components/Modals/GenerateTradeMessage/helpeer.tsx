import { Text, Image, Group, SelectProps } from "@mantine/core";

export const renderSelectOption: SelectProps["renderOption"] = ({ option, checked }) => (
  <Group gap="xs" style={{ fontWeight: checked ? 700 : 400 }} justify="flex-start">
    <span>
      <Image src={(option as any).img} fit="contain" width={20} height={20} />
    </span>
    <Text>{option.label}</Text>
  </Group>
);

export const RenderMessageWithIcons = (message: string, chatIcons: { code: string; url: string }[]) => {
  if (!chatIcons) return <Text size="sm">{message}</Text>;

  const iconMap = chatIcons.reduce((acc, icon) => {
    acc[icon.code] = { url: icon.url, code: icon.code };
    return acc;
  }, {} as Record<string, { url: string; code: string }>);

  // Split message by icon patterns (:iconname:)
  const parts: (string | { type: "icon"; url: string; code: string })[] = [];
  let lastIndex = 0;
  const iconRegex = /:([^:]+):/g;
  let match;

  while ((match = iconRegex.exec(message)) !== null) {
    // Add text before the icon
    if (match.index > lastIndex) {
      parts.push(message.substring(lastIndex, match.index));
    }

    // Add the icon
    const iconCode = `:${match[1]}:`;
    const iconData = iconMap[iconCode];
    if (iconData) {
      parts.push({ type: "icon", url: iconData.url, code: iconData.code });
    } else {
      parts.push(iconCode); // Keep original if icon not found
    }

    lastIndex = match.index + match[0].length;
  }

  // Add remaining text
  if (lastIndex < message.length) {
    parts.push(message.substring(lastIndex));
  }

  return (
    <Text size="sm" component="div" style={{ display: "flex", alignItems: "center", flexWrap: "wrap", gap: "2px" }}>
      {parts.map((part, index) => {
        if (typeof part === "string") {
          return (
            <span key={index} style={{ whiteSpace: "pre-wrap" }}>
              {part}
            </span>
          );
        } else {
          return (
            <img
              key={index}
              src={part.url}
              alt={part.code}
              style={{
                width: "18px",
                height: "18px",
                display: "inline-block",
                verticalAlign: "middle",
                margin: "0 1px",
              }}
            />
          );
        }
      })}
    </Text>
  );
};
