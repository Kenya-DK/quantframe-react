import { faArrowDown, faArrowUp } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { RingProgress, Text, Center, Group } from '@mantine/core';

interface StatsRingProps {
  label: string;
  stats: string;
  progress: number;
  color: string;
  icon: 'up' | 'down';
}

const icons = {
  up: faArrowUp,
  down: faArrowDown,
};

export function StatsRing({ label, stats, progress, color, icon }: StatsRingProps) {
  const Icon = icons[icon];

  return (
    <Group>
      <RingProgress
        size={80}
        roundCaps
        thickness={8}
        sections={[{ value: progress, color: color }]}
        label={
          <Center>
            <FontAwesomeIcon icon={Icon} />
          </Center>
        }
      />

      <div>
        <Text color="dimmed" size="xs" transform="uppercase" weight={700}>
          {label}
        </Text>
        <Text weight={700} size="xl">
          {stats}
        </Text>
      </div>
    </Group>
  );
}