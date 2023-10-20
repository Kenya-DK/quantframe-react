
import { Button, Progress, createStyles } from '@mantine/core';
import { useEffect, useState } from 'react';

interface ButtonProgressProps {
  current: number;
  max: number;
  label: string;
  progressLabel: string;
  onStart?: () => void;
}

const useStyles = createStyles((theme) => ({
  button: {
    position: 'relative',
    transition: 'background-color 150ms ease',
  },

  progress: {
    ...theme.fn.cover(-1),
    height: 'auto',
    backgroundColor: 'transparent',
    zIndex: 0,
  },

  label: {
    position: 'relative',
    zIndex: 1,
  },
}));
export const ButtonProgress = ({ label, progressLabel, current, max, onStart }: ButtonProgressProps) => {
  const { classes, theme } = useStyles();
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    setProgress((current / max) * 100);
  }, [current, max]);

  return (
    <Button
      loading={progress > 0}
      fullWidth
      className={classes.button}
      onClick={() => {
        // Progress is running
        onStart && onStart();
      }}
      color={theme.primaryColor}
    >
      <div className={classes.label}>
        {progress !== 0 ? progressLabel : label}
      </div>
      {progress !== 0 && (
        <Progress
          value={progress}
          className={classes.progress}
          color={theme.fn.rgba(theme.colors[theme.primaryColor][2], 0.35)}
          radius="sm"
        />
      )}
    </Button>

  );
}