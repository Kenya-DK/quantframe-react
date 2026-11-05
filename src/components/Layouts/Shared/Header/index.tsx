import { Group, useMantineTheme } from "@mantine/core";
import classes from "./Header.module.css";
import { Logo } from "../Logo";
import { Clock } from "../Clock";
import { UserMenu } from "../UserMenu";
import { useAppContext } from "@contexts/app.context";
import { Ticker } from "@components/Layouts/Shared/Ticker";
import { QuantframeApiTypes } from "$types";

export type HeaderProps = {
  /** Optional middle content (e.g. a search field) rendered between the logo and the user menu. */
  center?: React.ReactNode;
  /** Hide the clock (used by full-screen feature layouts like WF inventory "next"). */
  showClock?: boolean;
  /** When set, the user dropdown gains a "Back to original UI" item navigating to this path. */
  userMenuBackTo?: string;
};

export function Header({ center, showClock = true, userMenuBackTo }: HeaderProps) {
  const theme = useMantineTheme();
  const { alerts } = useAppContext();
  const handleAlertClick = (alert: QuantframeApiTypes.AlertDto) => {
    if (!alert.properties) return;
    const { event, payload } = alert.properties as { event: string; payload: any };
    if (!event) return;
    switch (event) {
      case "open_url":
        if (payload) open(payload);
        break;
      default:
        break;
    }
  };
  return (
    <>
      <Group ml={"sm"} mr={"sm"} justify="space-between" className={classes.header}>
        <Logo color={theme.other.logoColor} />
        {center}
        {showClock && <Clock />}
        <UserMenu backTo={userMenuBackTo} />
      </Group>
      <Ticker
        data={alerts.map((alert) => ({
          label: alert.context,
          props: {
            "data-alert-type": alert.type,
            "data-color-mode": "text",
          },
          onClick: alert.properties ? () => handleAlertClick(alert) : undefined,
        }))}
        loop
      />
    </>
  );
}

