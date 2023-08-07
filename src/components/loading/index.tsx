
import { Text, Box } from '@mantine/core';
import './index.css'
interface LoadingProps {
  text?: string;
}

export const Loading = (props: LoadingProps) => {
  const { text } = props;
  return (
    <Box sx={{ height: "100%", width: "100&" }}>
      {text &&
        <Text variant="h4" component="div" sx={{ textAlign: "center", color: "white" }}>
          {text}
        </Text>
      }
      <div className="loader">
        <div className="spin"></div>
        <div className="bounce"></div>
      </div>
    </Box >
  );
}