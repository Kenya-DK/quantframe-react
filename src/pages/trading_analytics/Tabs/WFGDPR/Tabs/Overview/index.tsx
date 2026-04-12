import { Avatar, Badge, Box, getGradient, Group, ScrollArea, SimpleGrid, Stack, Text, useMantineTheme } from "@mantine/core";
import { StatsWithIcon } from "@components/Shared/StatsWithIcon";
import { TauriTypes } from "$types";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faCalendar,
  faEnvelope,
  faGlobe,
  faLanguage,
  faShoppingCart,
  faExchangeAlt,
  faCoins,
  faNetworkWired,
} from "@fortawesome/free-solid-svg-icons";
import dayjs from "dayjs";
import { useTranslatePages } from "@hooks/useTranslate.hook";

interface OverviewPanelProps {
  value: TauriTypes.WFGDPRAccount | null;
}

export const OverviewPanel = ({ value }: OverviewPanelProps) => {
  const theme = useMantineTheme();

  if (!value)
    return (
      <Box p="xs" h="85vh">
        <Text c="dimmed">No account selected.</Text>
      </Box>
    );

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.tabs.wfgdpr.overview.${key}`, { ...context }, i18Key);

  const totalPlatinumTraded = value.trades.reduce((sum, t) => sum + t.platinum, 0);
  const totalSpent = value.purchases.reduce((sum, p) => sum + p.price, 0);
  const uniqueIPs = new Set(value.ips).size;

  return (
    <ScrollArea h="85vh">
      <Stack p="xs" gap="md">
        {/* Profile Header */}
        <StatsWithIcon
          icon={
            <Avatar size="lg" radius="xl" color="blue">
              {value.display_name.slice(0, 2).toUpperCase()}
            </Avatar>
          }
          title={useTranslate("labels.member_since")}
          count={dayjs(value.account_creation_date).format("DD MMM YYYY")}
          color="transparent"
          footer={
            <Group gap="xs" wrap="wrap">
              <Text size="xs" c="dimmed">
                <FontAwesomeIcon icon={faEnvelope} style={{ marginRight: 4 }} />
                {value.email}
              </Text>
              {value.subscribed_to_emails && <Badge color="teal">{useTranslate("labels.subscribed")}</Badge>}
              <Badge color="gray" leftSection={<FontAwesomeIcon icon={faGlobe} style={{ fontSize: 10 }} />}>
                {value.country_code}
              </Badge>
              <Badge color="gray" leftSection={<FontAwesomeIcon icon={faLanguage} style={{ fontSize: 10 }} />}>
                {value.language}
              </Badge>
              {value.signup_page && <Badge color="gray">via {value.signup_page}</Badge>}
            </Group>
          }
        />

        {/* Stats */}
        <SimpleGrid cols={4}>
          <StatsWithIcon
            icon={<FontAwesomeIcon icon={faExchangeAlt} />}
            title={useTranslate("labels.trades")}
            count={value.trades.length.toLocaleString()}
            color={getGradient({ from: theme.colors.cyan[7], to: theme.colors.teal[5], deg: 135 }, theme)}
          />
          <StatsWithIcon
            icon={<FontAwesomeIcon icon={faCoins} />}
            title={useTranslate("labels.platinum_traded")}
            count={totalPlatinumTraded.toLocaleString()}
            color={getGradient({ from: theme.colors.yellow[5], to: theme.colors.orange[5], deg: 135 }, theme)}
          />
          <StatsWithIcon
            icon={<FontAwesomeIcon icon={faShoppingCart} />}
            title={useTranslate("labels.purchases")}
            count={value.purchases.length.toLocaleString()}
            color={getGradient({ from: theme.colors.lime[6], to: theme.colors.green[6], deg: 135 }, theme)}
          />
          <StatsWithIcon
            icon={<FontAwesomeIcon icon={faCoins} />}
            title={useTranslate("labels.total_spent")}
            count={`${totalSpent.toLocaleString()} cr`}
            color={getGradient({ from: theme.colors.red[7], to: theme.colors.orange[6], deg: 135 }, theme)}
          />
          <StatsWithIcon
            icon={<FontAwesomeIcon icon={faExchangeAlt} />}
            title={useTranslate("labels.transactions")}
            count={value.transactions.length.toLocaleString()}
            color={getGradient({ from: theme.colors.grape[6], to: theme.colors.violet[5], deg: 135 }, theme)}
          />
          <StatsWithIcon
            icon={<FontAwesomeIcon icon={faNetworkWired} />}
            title={useTranslate("labels.unique_ips")}
            count={uniqueIPs}
            color={getGradient({ from: theme.colors.blue[9], to: theme.colors.indigo[8], deg: 135 }, theme)}
          />
          <StatsWithIcon
            icon={<FontAwesomeIcon icon={faCalendar} />}
            title={useTranslate("labels.total_logins")}
            count={value.logins.length.toLocaleString()}
            color={getGradient({ from: theme.colors.pink[6], to: theme.colors.red[5], deg: 135 }, theme)}
          />
        </SimpleGrid>
      </Stack>
    </ScrollArea>
  );
};
