import { Group, useMantineTheme } from "@mantine/core";
import classes from "./Header.module.css";
import { Logo } from "../Logo";
import { Clock } from "../Clock";
import { UserMenu } from "../UserMenu";

export type HeaderProps = {};

export function Header({}: HeaderProps) {
  const theme = useMantineTheme();
  return (
    <Group ml={"sm"} mr={"sm"} justify="space-between" className={classes.header}>
      <Logo color={theme.other.logoColor} />
      <Clock />
      <UserMenu />
    </Group>
  );
}
