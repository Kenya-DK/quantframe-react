
import { Button, Center, Stack, Title } from '@mantine/core';
import { useStatsScraperContext } from '../contexts/statsScraper.context';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faDatabase } from '@fortawesome/free-solid-svg-icons';
import { useLiveScraperContext } from '../contexts/liveScraper.context';
import { useWhisperScraperContext } from '../contexts';
export const TransactionControl = () => {
  const { isRunning: statsIsRunning, run: runStatsScraper } = useStatsScraperContext();
  const { isRunning: liveIsRunning, toggle: toggleLiveScraper } = useLiveScraperContext();
  const { isStarting: isWhisperStarting, isRunning: whisperIsRunning, toggle: toggleWhisperScraper } = useWhisperScraperContext();
  return (
    <Center >
      <Stack w={"50%"}>
        <Title order={1}>Transaction Control</Title>
        <Button leftIcon={<FontAwesomeIcon icon={faDatabase} />} loading={statsIsRunning} onClick={() => runStatsScraper()}>
          Refresh Price History
        </Button>
        <Button color={liveIsRunning ? "red.7" : "green.7"} leftIcon={<FontAwesomeIcon icon={faDatabase} />} onClick={() => toggleLiveScraper()} disabled={statsIsRunning}>
          {liveIsRunning ? "Stop" : "Start"} Live Scraper
        </Button>
        <Button color={whisperIsRunning ? "red.7" : "green.7"} leftIcon={<FontAwesomeIcon icon={faDatabase} />} onClick={() => toggleWhisperScraper()} disabled={statsIsRunning || isWhisperStarting}>
          {whisperIsRunning ? "Stop" : "Start"} Whisper Linstener
        </Button>
      </Stack>
    </Center>
  );
}