import './index.css'
import { Center, keyframes, useMantineTheme } from "@mantine/core";
interface InfoCardProps {
  opened: boolean;
}

export const fadeOut = keyframes({
  '0%, 85%': { opacity: '1' },
  '100%': { opacity: '0' },
});



export const zoomIn = keyframes({
  'from, 20%, 53%, 80%, to': { transform: 'translate3d(0, 0, 0)' },
  '40%, 43%': { transform: 'translate3d(0, -1.875rem, 0)' },
  '70%': { transform: 'translate3d(0, -0.9375rem, 0)' },
  '90%': { transform: 'translate3d(0, -0.25rem, 0)' },
});
export const SplashScreen = (props: InfoCardProps) => {
  const { opened } = props;
  const theme = useMantineTheme();
  return (
    <>
      <Center className={`cover ${!opened ? "hide" : ""}`}
        style={{ backgroundColor: theme.colors.dark[7] }}
      >
        <img src="/app-icon.png" alt="logo" id="icon" />
      </Center>
    </>
  );

}