
import { Button, Center, Stack, Title } from '@mantine/core';
import { usePriceScraperContext } from '../contexts/priceScraper.context';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faDatabase } from '@fortawesome/free-solid-svg-icons';
import { useLiveScraperContext } from '../contexts/liveScraper.context';
import { useWhisperScraperContext } from '../contexts';
import { ButtonProgress } from './buttonProgress';
import { useTranslateComponent } from '@hooks/index';
import api from '../api';
const days = 15;
export const TransactionControl = () => {
  const { is_running: statsIsRunning, max, current } = usePriceScraperContext();
  const { is_running: liveIsRunning } = useLiveScraperContext();
  const { is_running: whisperIsRunning } = useWhisperScraperContext();
  const useTranslate = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`transactioncontrol.${key}`, { ...context })

  return (
    <Center >
      <Stack >
        <Title order={1}>{useTranslate("title")}</Title>
        <ButtonProgress
          onStart={async () => {
            await api.price_scraper.start_scraper(days);
          }}
          max={max == 0 ? 1 : max}
          current={current}
          label={useTranslate("price_scraper_start")}
          progressLabel={useTranslate("price_scraper_running")}
        />
        <Button color={liveIsRunning ? "red.7" : "green.7"} leftIcon={<FontAwesomeIcon icon={faDatabase} />} onClick={async () => await api.live_scraper.start_scraper()} disabled={statsIsRunning}>
          {liveIsRunning ? useTranslate("live_trading_stop") : useTranslate("live_trading_start")}
        </Button>
        <Button color={whisperIsRunning ? "red.7" : "green.7"} leftIcon={<FontAwesomeIcon icon={faDatabase} />} onClick={async () => await api.whisper_scraper.start_scraper()} disabled={statsIsRunning}>
          {whisperIsRunning ? useTranslate("wisper_stop") : useTranslate("wisper_start")}
        </Button>
      </Stack>
    </Center>
  );
}