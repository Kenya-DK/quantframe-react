
import { Button, Center, Group, Stack } from '@mantine/core';
import { usePriceScraperContext } from '../contexts/priceScraper.context';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faDatabase } from '@fortawesome/free-solid-svg-icons';
import { useLiveScraperContext } from '../contexts/liveScraper.context';
import { ButtonProgress } from './buttonProgress';
import { useTranslateComponent } from '@hooks/index';


import api from '../api';
import { TextColor } from './textColor';
import dayjs from 'dayjs';
const days = 15;
export const TransactionControl = () => {
  const { is_running: statsIsRunning, max, current, last_run } = usePriceScraperContext();
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
            <ButtonProgress
              onStart={async () => await api.price_scraper.start_scraper(days)}
              max={max == 0 ? 1 : max}
              current={current}
              label={useTranslate("price_scraper_start")}
              progressLabel={useTranslate("price_scraper_running")}
            />
          </Group>
          <Group position="center" spacing="xs">
            <Button color={liveIsRunning ? "red.7" : "green.7"} leftIcon={<FontAwesomeIcon icon={faDatabase} />} onClick={async () => {
              await api.live_scraper.start_scraper()
            }} disabled={statsIsRunning || last_run == null}>
              {liveIsRunning ? useTranslate("live_trading_stop") : useTranslate("live_trading_start")}
            </Button>
          </Group>
        </Group>
        <TextColor i18nKey="components.transactioncontrol.price_scraper_last_run" values={{ date: last_run == null ? "N/A" : dayjs(last_run).format("DD/MM/YYYY HH:mm") }} />
        {message && <TextColor i18nKey={`progress.${message.i18n_key}`} values={{ ...message.values }} />}
      </Stack>
    </Center>
  );
}