import { Text, Paper, Box, Divider, Group, Center, Stack, Title } from '@mantine/core';
import React from 'react';

interface StatsWithIconProps {
  count: number | string;
  color: string;
  title: string;
  icon: React.ReactNode;
  fotter?: React.ReactNode;
}

export function StatsWithIcon({ count, color, title, fotter, icon }: StatsWithIconProps) {

  return (
    <Box sx={{ paddingTop: "24px" }}>
      <Paper sx={{ padding: "1rem", }} >
        <Group position="apart" noWrap>
          <Center mt={-60}
            display="flex"
            sx={{
              width: "4rem",
              height: "4rem",
              background: color,
              borderRadius: "15%",
            }}>
            {icon}
          </Center>
          <Stack align="center" spacing={"1"} mb={15}>
            <Title order={5}>{title}</Title>
            <Text variant="body1" color="text.secondary">
              {count}
            </Text>
          </Stack>
        </Group>
        {fotter && (<Divider />)}
        {fotter && (
          <Group mt={5} >
            {fotter}
          </Group>
        )}
      </Paper>
    </Box>
  );
}