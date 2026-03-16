import { isValidElement, memo } from "react";
import { alpha, Card, Group, CardProps } from "@mantine/core";
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

const slotConfig: Record<SlotKey, { justify: "flex-start" | "center" | "flex-end"; flex: string }> = {
  headerLeft: { justify: "flex-start", flex: "0 1 auto" },
  headerCenter: { justify: "center", flex: "1 1 auto" },
  headerRight: { justify: "flex-end", flex: "0 1 auto" },

  footerLeft: { justify: "flex-start", flex: "1" },
  footerCenter: { justify: "center", flex: "1" },
  footerRight: { justify: "flex-end", flex: "1" },
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

    return (
      <Group justify="space-between" align="center" wrap="nowrap">
        {keys.map((slot) => {
          const config = slotConfig[slot];

          return (
            <div
              key={slot}
              style={{
                flex: config.flex,
                display: "flex",
                justifyContent: config.justify,
                minWidth: 0,
              }}
            >
              {renderSlot(slot)}
            </div>
          );
        })}
      </Group>
    );
  };

  return (
    <Card radius="md" ref={ref} pos="relative" style={{ display: "flex", flexDirection: "column", ...style }} {...cardProps}>
      {/* Header */}
      {slots.headerLeft || slots.headerCenter || slots.headerRight ? (
        <Card.Section bg={alpha("var(--mantine-color-dark-7)", 0.7)} p={3}>
          {renderSection("header")}
        </Card.Section>
      ) : null}

      {/* Body */}
      <Card.Section p="sm" style={{ flexGrow: 1 }}>
        {renderBody ? renderBody(value) : null}
      </Card.Section>

      {/* Footer */}
      {slots.footerLeft || slots.footerCenter || slots.footerRight ? (
        <Card.Section bg={alpha("var(--mantine-color-dark-7)", 0.7)} p={3}>
          {renderSection("footer")}
        </Card.Section>
      ) : null}
    </Card>
  );
});
