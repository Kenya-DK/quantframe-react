import { Paper, Text, Group, ThemeIcon, NumberFormatter } from "@mantine/core";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faArrowTrendDown, faArrowTrendUp } from "@fortawesome/free-solid-svg-icons";
export interface MetricCardProps {
  icon: React.ReactNode;
  title: string;
  value: number | string;
  suffix?: string;
  prefix?: string;
  color?: string;
  isPercentage?: boolean;
  isCurrency?: boolean;
  trend?: "up" | "down" | "neutral";
}
export const MetricCard = ({
  icon,
  title,
  value,
  suffix,
  prefix,
  color = "blue",
  isPercentage = false,
  isCurrency = false,
  trend = "neutral",
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

    if (isCurrency) {
      // Use condensed format on smaller screens for better readability
      const isLargeNumber = Math.abs(value as number) >= 10000;
      if (isLargeNumber && window.innerWidth < 768) {
        return `${formatLargeNumber(value as number)} Pl`;
      }
      return <NumberFormatter value={value} thousandSeparator="." decimalScale={2} decimalSeparator="," suffix=" Pl" />;
    }

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
    <Paper p="md" withBorder h="100%">
      <Group justify="space-between" mb="xs" align="flex-start">
        <ThemeIcon size="lg" variant="light" color={getTrendColor()}>
          {icon}
        </ThemeIcon>
        {trend !== "neutral" && (
          <ThemeIcon size="sm" variant="light" color={getTrendColor()}>
            {trend === "up" ? <FontAwesomeIcon icon={faArrowTrendUp} /> : <FontAwesomeIcon icon={faArrowTrendDown} />}
          </ThemeIcon>
        )}
      </Group>

      <Text size="sm" c="dimmed" mb="xs" style={{ flex: 1 }}>
        {title}
      </Text>

      <Text fw={700} c={getTrendColor()}>
        {prefix}
        {renderValue()}
        {suffix}
      </Text>
    </Paper>
  );
};
