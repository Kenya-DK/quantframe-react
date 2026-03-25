import { isValidElement, memo } from "react";
import { alpha, Card, CardProps } from "@mantine/core";
import { useHover } from "@mantine/hooks";
import { TextTranslate, TextTranslateProps } from "@components/Shared/TextTranslate";
type SlotKey = "headerLeft" | "headerCenter" | "headerRight" | "footerLeft" | "footerCenter" | "footerRight";

interface TextTranslatePropsExtended extends TextTranslateProps {
  hide?: boolean;
}

interface PreviewCardProps<T = any> extends CardProps {
  value: T;

  headerLeft?: TextTranslatePropsExtended | React.ReactNode;
  headerCenter?: TextTranslatePropsExtended | React.ReactNode;
  headerRight?: TextTranslatePropsExtended | React.ReactNode;

  footerLeft?: TextTranslatePropsExtended | React.ReactNode;
  footerCenter?: TextTranslatePropsExtended | React.ReactNode;
  footerRight?: TextTranslatePropsExtended | React.ReactNode;

  renderBody?: (value: T) => React.ReactNode;
}

const slotConfig: Record<SlotKey, { justify: "flex-start" | "center" | "flex-end"; gridColumn: string }> = {
  headerLeft: { justify: "flex-start", gridColumn: "1" },
  headerCenter: { justify: "center", gridColumn: "2" },
  headerRight: { justify: "flex-end", gridColumn: "3" },

  footerLeft: { justify: "flex-start", gridColumn: "1" },
  footerCenter: { justify: "center", gridColumn: "2" },
  footerRight: { justify: "flex-end", gridColumn: "3" },
};

export const PreviewCard = memo(function PreviewCard<T>({
  value,
  renderBody,
  headerLeft,
  headerCenter,
  headerRight,
  footerLeft,
  footerCenter,
  footerRight,

  style,
  ...cardProps
}: PreviewCardProps<T>) {
  const { ref } = useHover();

  if (!value) return <>...</>;

  const slots = {
    headerLeft,
    headerCenter,
    headerRight,
    footerLeft,
    footerCenter,
    footerRight,
  };

  const renderSlot = (slot: SlotKey) => {
    const slotValue = slots[slot as keyof typeof slots] as TextTranslatePropsExtended | React.ReactNode | undefined;

    if (slotValue == null || slotValue === false) {
      return null;
    }

    if (isValidElement(slotValue) || typeof slotValue !== "object" || Array.isArray(slotValue)) {
      return slotValue as React.ReactNode;
    }

    if (!("i18nKey" in slotValue)) {
      return null;
    }

    const translateProps = slotValue as TextTranslatePropsExtended;
    const { hide, ...textTranslateProps } = translateProps;

    if (hide) {
      return null;
    }

    return <TextTranslate {...textTranslateProps} />;
  };

  const renderSection = (section: "header" | "footer") => {
    const keys = (["Left", "Center", "Right"] as const).map((pos) => `${section}${pos}` as SlotKey);
    const renderedSlots = keys.map((slot) => ({
      slot,
      content: renderSlot(slot),
    }));

    const leftSlot = renderedSlots.find(({ slot }) => slot === `${section}Left`)!;
    const centerSlot = renderedSlots.find(({ slot }) => slot === `${section}Center`)!;
    const rightSlot = renderedSlots.find(({ slot }) => slot === `${section}Right`)!;

    const hasLeft = leftSlot.content !== null && leftSlot.content !== false && leftSlot.content !== "";
    const hasRight = rightSlot.content !== null && rightSlot.content !== false && rightSlot.content !== "";
    const hasCenter = centerSlot.content !== null && centerSlot.content !== false && centerSlot.content !== "";
    const isCenterOnly = hasCenter && !hasLeft && !hasRight;

    return (
      <div
        style={{
          display: isCenterOnly ? "flex" : "grid",
          gridTemplateColumns: isCenterOnly ? undefined : "minmax(0, 1fr) auto minmax(0, 1fr)",
          alignItems: "center",
          gap: 8,
        }}
      >
        {renderedSlots.map(({ slot, content }) => {
          if (isCenterOnly && slot !== `${section}Center`) {
            return null;
          }

          const config = slotConfig[slot];

          return (
            <div
              key={slot}
              style={{
                gridColumn: isCenterOnly ? undefined : config.gridColumn,
                display: "flex",
                justifyContent: config.justify,
                minWidth: 0,
                overflow: "hidden",
                width: isCenterOnly ? "100%" : undefined,
              }}
            >
              {content}
            </div>
          );
        })}
      </div>
    );
  };

  const hasRenderableSlots = (section: "header" | "footer") => {
    const keys = (["Left", "Center", "Right"] as const).map((pos) => `${section}${pos}` as SlotKey);

    return keys.some((slot) => {
      const content = renderSlot(slot);
      return content !== null && content !== false && content !== "";
    });
  };

  return (
    <Card radius="md" ref={ref} pos="relative" style={{ display: "flex", flexDirection: "column", ...style }} {...cardProps}>
      {/* Header */}
      {hasRenderableSlots("header") ? (
        <Card.Section bg={alpha("var(--mantine-color-dark-7)", 0.7)} p={3}>
          {renderSection("header")}
        </Card.Section>
      ) : null}

      {/* Body */}
      <Card.Section p="sm" style={{ flexGrow: 1 }}>
        {renderBody ? renderBody(value) : null}
      </Card.Section>

      {/* Footer */}
      {hasRenderableSlots("footer") ? (
        <Card.Section bg={alpha("var(--mantine-color-dark-7)", 0.7)} p={3}>
          {renderSection("footer")}
        </Card.Section>
      ) : null}
    </Card>
  );
});
