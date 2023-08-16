import { createStyles, rem, Tooltip, UnstyledButton, Navbar, Stack } from "@mantine/core";
import { ReactNode, useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faChartSimple, faGlobe, faHome } from "@fortawesome/free-solid-svg-icons";

interface NavbarLinkProps {
  icon: ReactNode;
  label: string;
  active?: boolean;
  onClick?(): void;
}
function NavbarLink({ icon, label, active, onClick }: NavbarLinkProps) {
  const { classes, cx } = useStyles();
  return (
    <Tooltip label={label} position="right" transitionProps={{ duration: 0 }}>
      <UnstyledButton onClick={onClick} className={cx(classes.link, { [classes.active]: active })}>
        {icon}
      </UnstyledButton>
    </Tooltip>
  );
}

const useStyles = createStyles((theme) => ({
  link: {
    width: rem(50),
    height: rem(50),
    borderRadius: theme.radius.md,
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    color: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.colors.gray[7],
    marginBottom: theme.spacing.xs,
    '&:hover': {
      backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[5] : theme.colors.gray[0],
    },
  },

  active: {
    '&, &:hover': {
      backgroundColor: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).background,
      color: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).color,
    },
  },
}));

export default function SideBar({ }) {
  const [active, setActive] = useState(2);

  const mockdata = [
    { icon: <FontAwesomeIcon icon={faHome} />, label: 'Home' },
    { icon: <FontAwesomeIcon icon={faGlobe} />, label: 'Live Trading' },
    { icon: <FontAwesomeIcon icon={faChartSimple} />, label: 'Statistics' },
    { icon: <FontAwesomeIcon icon={faChartSimple} />, label: 'Warframe Market' },
  ];
  const links = mockdata.map((link, index) => (
    <NavbarLink
      {...link}
      key={link.label}
      active={index === active}
      onClick={() => setActive(index)}
    />
  ));
  return (
    <Navbar width={{ base: 70 }} p="xs">
      <Navbar.Section grow>
        <Stack justify="center" spacing={0}>
          {links}
        </Stack>
      </Navbar.Section>
    </Navbar>
  )
}