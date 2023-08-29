import './App.css'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ModalsProvider } from '@mantine/modals';
import { createStyles } from '@mantine/core';
import { PromptModal } from './components/modals/prompt.modal';
import { TauriContextProvider } from './contexts';
// Create a client
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
    },
  },
})


const useStyles = createStyles(() => ({
  header: {
    borderBottom: `1px gray solid `,
    padding: 10,
  },
}));

const modals = {
  prompt: PromptModal
  /* ...other modals */
};
declare module '@mantine/modals' {
  export interface MantineModalsOverride {
    modals: typeof modals;
  }
}

function App() {
  const { classes } = useStyles();
  return (
    <QueryClientProvider client={queryClient}>
      <ModalsProvider
        modals={modals}
        modalProps={{
          centered: true,
          classNames: classes,
          onClose() {
            console.log("Modal closed");
          },
        }}>
        <TauriContextProvider>
          {/* <StatsScraperContextProvider>
            <LiveScraperContextProvider>
              <WhisperScraperContextProvider>
                <AppRoutes />
              </WhisperScraperContextProvider>
            </LiveScraperContextProvider>
          </StatsScraperContextProvider> */}
        </TauriContextProvider>
        {/* <ReactQueryDevtools initialIsOpen={false} /> */}
      </ModalsProvider>
    </QueryClientProvider>
  )
}

export default App
