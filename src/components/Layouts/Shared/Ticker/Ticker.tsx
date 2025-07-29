import { Box, Group, Paper, Text } from "@mantine/core";
import classes from "./Ticker.module.css";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faMousePointer } from "@fortawesome/free-solid-svg-icons";

export interface TickerItemProps {
  label: string;
  props?: { [key: string]: any };
  onClick?: (e: React.MouseEvent<HTMLDivElement>) => void;
}

export type TickerProps = {
  data: TickerItemProps[];
  component?: any;
  keyName?: string;
  speed?: number;
  delay?: number;
  direction?: "normal" | "reverse" | "alternate" | "alternate-reverse";
  tickerClassName?: string;
  itemClassName?: string;
  tickerTextClassName?: string;
  tickerStyle?: { [key: string]: any };
  itemStyle?: { [key: string]: any };
  loop?: boolean;
};

export function Ticker({ loop, speed, itemStyle, direction, delay, tickerClassName, data, tickerStyle }: TickerProps) {
  return (
    <Paper className={[classes.newsTicker, tickerClassName].join(" ")} style={tickerStyle}>
      <Box
        className={classes.tickerContent}
        style={{
          animationDuration: `${data.length * (61 - (speed || 100))}s`,
          animationDelay: `${delay}s`,
          animationDirection: direction,
          animationIterationCount: loop ? "infinite" : 1,
        }}
      >
        {data.map((item, index) => {
          return (
            <Group
              data-clickable={!!item.onClick}
              onClick={item.onClick}
              key={index}
              className={classes.tickerItem}
              style={itemStyle}
              {...item.props}
            >
              <Text fw={700} fs={"14"} className={classes.tickerText}>
                {item.onClick && <FontAwesomeIcon icon={faMousePointer} />}
                {item.label}
              </Text>
            </Group>
          );
        })}
      </Box>
    </Paper>
  );
}
