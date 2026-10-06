import { Button } from "@mantine/core";

export type ButtonIntervalProps = {
  intervals: number[];
  disabled?: boolean;
  prefix: string;
  color: string;
  OnClick: (interval: number) => void;
};

export function ButtonInterval({ color, prefix, OnClick, intervals, disabled }: ButtonIntervalProps) {
  return (
    <>
      {intervals.map((interval) => (
        <Button key={interval} p="0px 11px 0px" h={22} onClick={() => OnClick(interval)} variant="filled" color={color} disabled={disabled}>
          {prefix}
          {interval}
        </Button>
      ))}
    </>
  );
}
