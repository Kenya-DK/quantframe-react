
import { Button, Center, Group, Stack } from '@mantine/core';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faDatabase } from '@fortawesome/free-solid-svg-icons';
import { useLiveScraperContext } from '../contexts/liveScraper.context';
import { useTranslateComponent } from '@hooks/index';


import api from '../api';
import { TextColor } from './textColor';
export const TransactionControl = () => {
  const { is_running: liveIsRunning, message } = useLiveScraperContext();
  const useTranslate = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`transactioncontrol.${key}`, { ...context })
  return (
    <Center >
      <Stack spacing={"1"} sx={{
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        alignItems: "center",
      }}>
        <Group grow position="center">
          <Group position="center" spacing="xs">
            <Button color={liveIsRunning ? "red.7" : "green.7"} leftIcon={<FontAwesomeIcon icon={faDatabase} />} onClick={async () => {
              await api.live_scraper.start_scraper()
            }} >
              {liveIsRunning ? useTranslate("live_trading_stop") : useTranslate("live_trading_start")}
            </Button>
          </Group>
        </Group>
        {message && <TextColor i18nKey={`progress.${message.i18n_key}`} values={{ ...message.values }} />}
      </Stack>
    </Center>
  );
}