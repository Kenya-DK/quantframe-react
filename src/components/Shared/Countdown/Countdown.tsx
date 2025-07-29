import { Text } from "@mantine/core";
import { useEffect, useState } from "react";
import { calculateTimeLeft, getTimeLeftString, TimeSpan } from "@utils/helper";
export interface CountdownProps {
  startDate: Date;
  endDate: Date;
  text?: string | React.ReactNode;
  showBackgroundColor?: boolean;
  onFinish?: () => void;
  onTick?: (progress: TimeSpan) => void;
}
export function Countdown({ startDate, endDate, text, onFinish, onTick }: CountdownProps) {
  const [timeLeft, setTimeLeft] = useState<TimeSpan>(calculateTimeLeft(endDate));
  const [, setColor] = useState<string>("#0288d1");

  useEffect(() => {
    const intervalId = setInterval(() => {
      const timeLeftNe = calculateTimeLeft(endDate);
      setTimeLeft(timeLeftNe);
      const percent = (timeLeftNe.totalSeconds / ((endDate.getTime() - startDate.getTime()) / 1000)) * 100;
      if (percent >= 50) setColor("#689f38");
      else if (percent < 49 && percent > 24) setColor("#ad6200");
      else if (percent < 24 && percent >= 0) setColor("#6d0700");
      else setColor("#0288d1");
      if (onTick) onTick(timeLeftNe);
      if (timeLeftNe.totalSeconds === 0) if (onFinish) onFinish();
    }, 1000);
    return () => clearInterval(intervalId);
  }, []);
  return (
    <Text component="span">
      {text} {getTimeLeftString(timeLeft)}
    </Text>
  );
}
