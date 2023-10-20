
import { Button, Center, Group } from '@mantine/core';
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
      <Group position="center" spacing="xs" mr={12}>
        <ButtonProgress
          onStart={async () => {
            await api.price_scraper.start_scraper(days);
          }}
          max={max == 0 ? 1 : max}
          current={current}
          label={useTranslate("price_scraper_start")}
          progressLabel={useTranslate("price_scraper_running")}
        />
      </Group>
      <Group position="center" spacing="xs" mr={12}>
        <Button color={liveIsRunning ? "red.7" : "green.7"} leftIcon={<FontAwesomeIcon icon={faDatabase} />} onClick={async () => await api.live_scraper.start_scraper()} disabled={statsIsRunning}>
          {liveIsRunning ? useTranslate("live_trading_stop") : useTranslate("live_trading_start")}
        </Button>
      </Group>
      <Group position="center" spacing="xs" mr={12}>
        <Button color={whisperIsRunning ? "red.7" : "green.7"} leftIcon={<FontAwesomeIcon icon={faDatabase} />} onClick={async () => await api.whisper_scraper.start_scraper()} disabled={statsIsRunning}>
          {whisperIsRunning ? useTranslate("wisper_stop") : useTranslate("wisper_start")}
        </Button>
      </Group>
    </Center>
  );
}