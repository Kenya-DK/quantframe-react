import React, { useState } from 'react';
import { Tooltip, UnstyledButton, Stack } from '@mantine/core';
import classes from './NavbarMinimalColored.module.css';

export type NavbarLinkProps = {
  icon: React.ReactNode;
  label: string;
  link: string;
  active?: boolean;
  align?: string;
  web?: boolean;
  onClick?(e: NavbarLinkProps): void;
}

function NavbarLink(props: NavbarLinkProps) {
  const { icon: Icon, label, active, onClick } = props;
  return (
    <Tooltip label={label} position="right" transitionProps={{ duration: 0 }}>
      <UnstyledButton onClick={() => { onClick && onClick(props); }} className={classes.link} data-active={active || undefined} data-rainbow-bg={true}>
        {Icon}
      </UnstyledButton>
    </Tooltip>
  );
}

export interface NavbarMinimalColoredProps {
  links: NavbarLinkProps[];
}

export function NavbarMinimalColored({ links }: NavbarMinimalColoredProps) {
  const [active, setActive] = useState(0);


  const GetActiveLinkByAlign = (align: 'top' | 'bottom') => {
    if (!links) return <></>;

    return links.filter((link) => link.align == align).map((link, index) => (
      <NavbarLink
        {...link}
        key={link.label}
        active={index === active && !link.web}
        onClick={() => {
          setActive(index);
          link.onClick && link.onClick(link);
        }}
      />
    ));
  }


  return (
    <nav className={classes.navbar}>
      <div className={classes.navbarMain}>
        <Stack justify="center" gap={3}>
          {GetActiveLinkByAlign('top')}
        </Stack>
      </div>

      <Stack justify="center" gap={0}>
        {GetActiveLinkByAlign('bottom')}
      </Stack>
    </nav>
  );
}