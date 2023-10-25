import './App.css'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ModalsProvider } from '@mantine/modals';
import { createStyles } from '@mantine/core';
import { PromptModal } from './components/modals/prompt.modal';
import { LiveScraperContextProvider, PriceScraperContextProvider, StockContextProvider, WhisperScraperContextProvider } from './contexts';
import AppRoutes from './layouts/routes';
import { AppContextProvider } from './contexts/app.context';
import { AuthContextProvider } from './contexts/auth.context';
import { CacheContextProvider } from './contexts/cache.context';
import { WarframeMarketContextProvider } from './contexts/warframeMarket.context';
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
        <AppContextProvider>
          <AuthContextProvider>
            <CacheContextProvider>
              <StockContextProvider>
                <WarframeMarketContextProvider>
                  <PriceScraperContextProvider>
                    <LiveScraperContextProvider>
                      <WhisperScraperContextProvider>
                        <AppRoutes />
                      </WhisperScraperContextProvider>
                    </LiveScraperContextProvider>
                  </PriceScraperContextProvider>
                </WarframeMarketContextProvider>
              </StockContextProvider>
            </CacheContextProvider>
          </AuthContextProvider>
        </AppContextProvider>
        {/* <ReactQueryDevtools initialIsOpen={false} /> */}
      </ModalsProvider>
    </QueryClientProvider>
  )
}

export default App

