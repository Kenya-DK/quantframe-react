import { Text, Group, ThemeIcon, NumberFormatter, Card } from "@mantine/core";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faArrowTrendDown, faArrowTrendUp } from "@fortawesome/free-solid-svg-icons";
import { DisplayPlatinum } from "../../DataDisplay/DisplayPlatinum";
export interface MetricCardProps {
  icon?: React.ReactNode;
  title: string;
  value: number | string;
  subValue?: string;
  subLabel?: string;
  suffix?: string;
  prefix?: string;
  color?: string;
  isPercentage?: boolean;
  isCurrency?: boolean;
  textTransform?: React.CSSProperties["textTransform"];
  trend?: "up" | "down" | "neutral";
}
export const MetricCard = ({
  icon,
  title,
  value,
  suffix,
  prefix,
  subValue,
  subLabel,
  color,
  isPercentage = false,
  isCurrency = false,
  trend = "neutral",
  textTransform = "uppercase",
}: MetricCardProps) => {
  const getTrendColor = () => {
    switch (trend) {
      case "up":
        return "var(--qf-positive-color)";
      case "down":
        return "var(--qf-negative-color)";
      default:
        return color;
    }
  };

  const renderValue = () => {
    if (typeof value === "string") return value;

    // For better mobile display, show condensed format for large numbers
    const formatLargeNumber = (num: number) => {
      if (Math.abs(num) >= 1000000) {
        return (num / 1000000).toFixed(1) + "M";
      }
      if (Math.abs(num) >= 10000) {
        return (num / 1000).toFixed(1) + "K";
      }
      return num.toString();
    };

    if (isCurrency) return <DisplayPlatinum value={value as number} iconColor="gray" />;

    if (isPercentage) {
      return <NumberFormatter value={value} decimalScale={2} suffix="%" />;
    }

    // Use condensed format for large counts on mobile
    const isLargeCount = (value as number) >= 10000;
    if (isLargeCount && window.innerWidth < 768) {
      return formatLargeNumber(value as number);
    }

    return <NumberFormatter value={value} thousandSeparator="." decimalScale={2} decimalSeparator="," />;
  };

  return (
    <Card padding="md" radius="md">
      <Group justify="space-between" mb="xs" align="flex-start">
        <Text size="xs" c="dimmed" fw={700} tt={textTransform}>
          {title}
        </Text>
        <ThemeIcon size="lg" variant="light" color={getTrendColor()} style={{ display: icon ? "" : "none" }}>
          {icon}
        </ThemeIcon>
      </Group>

      <Group gap={2}>
        <Text fw={700} fz="xl" c={getTrendColor()}>
          {prefix}
          {renderValue()}
          {suffix}
        </Text>
        {trend !== "neutral" && (
          <ThemeIcon size="sm" variant="light" color={getTrendColor()}>
            {trend === "up" ? <FontAwesomeIcon icon={faArrowTrendUp} /> : <FontAwesomeIcon icon={faArrowTrendDown} />}
          </ThemeIcon>
        )}
      </Group>
      <Text size="xs" c="dimmed" mt={4} style={{ display: subValue && subLabel ? "" : "none" }}>
        {subLabel}: {subValue}
      </Text>
    </Card>
  );
};
