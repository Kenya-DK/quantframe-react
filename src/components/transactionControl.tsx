
import { Button, Center, Stack, Title } from '@mantine/core';
import { useStatsScraperContext } from '../contexts/statsScraper.context';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faDatabase } from '@fortawesome/free-solid-svg-icons';
import { useLiveScraperContext } from '../contexts/liveScraper.context';
import { useWhisperScraperContext } from '../contexts';
import { useEffect, useState } from 'react';
import { ButtonProgress } from './buttonProgress';
import { OffTauriEvent, OnTauriEvent } from '../utils';

const days = 8;
export const TransactionControl = () => {
  const { isRunning: statsIsRunning, run: runStatsScraper } = useStatsScraperContext();
  const { isRunning: liveIsRunning, toggle: toggleLiveScraper } = useLiveScraperContext();
  const { isStarting: isWhisperStarting, isRunning: whisperIsRunning, toggle: toggleWhisperScraper } = useWhisperScraperContext();
  const [price_scraper_process, setPriceScraperProcess] = useState<number>(0);
  const useTranslate = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`transactioncontrol.${key}`, { ...context })

  useEffect(() => {
    OnTauriEvent("price_scraper_update_progress", (data: { current: number }) => {
      const { current } = data;
      setPriceScraperProcess(current);
    });
    OnTauriEvent("price_scraper_update_complete", () => {
      setPriceScraperProcess(0);
    });
    return () => {
      OffTauriEvent("price_scraper_update_progress");
      OffTauriEvent("price_scraper_update_complete");
    }
  }, []);
  return (
    <Center >
      <Stack w={"50%"}>
        <Title order={1}>{useTranslate("title")}</Title>
        <ButtonProgress
          onStart={() => {
            setPriceScraperProcess(0.1);
            runStatsScraper(days);
          }}
          max={8}
          current={price_scraper_process}
          label={useTranslate("price_scraper_start")}
          progressLabel={useTranslate("price_scraper_running")}
        />
        <Button color={liveIsRunning ? "red.7" : "green.7"} leftIcon={<FontAwesomeIcon icon={faDatabase} />} onClick={() => toggleLiveScraper()} disabled={statsIsRunning}>
          {liveIsRunning ? useTranslate(useTranslate("live_trading_stop")) : useTranslate("live_trading_start")}
        </Button>
        <Button color={whisperIsRunning ? "red.7" : "green.7"} leftIcon={<FontAwesomeIcon icon={faDatabase} />} onClick={() => toggleWhisperScraper()} disabled={statsIsRunning || isWhisperStarting}>
          {whisperIsRunning ? useTranslate("wisper_stop") : useTranslate("wisper_start")}
        </Button>
      </Stack>
    </Center>
  );
}