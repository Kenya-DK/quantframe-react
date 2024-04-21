import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ModalsProvider } from '@mantine/modals';
import { AppContextProvider, LiveScraperContextProvider, StockContextProvider, WarframeMarketContextProvider } from './contexts';
import { AppRoutes, PromptModal } from '@components';
import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import { en } from './lang/en'
import { dk } from './lang/dk'

import { IconDefinition, library, dom, IconPrefix, IconName } from '@fortawesome/fontawesome-svg-core'
var faSplat: IconDefinition = {
  prefix: 'fac' as IconPrefix,
  iconName: 'splat' as IconName,
  icon: [
    448, 448,
    [],
    "",
    'M163.006,417.598 L166.015,306.629 L63.506,343.841 L129.87,255.871 L25.5,224.5 L129.87,193.129 L63.506,105.159 L166.015,142.371 L163.006,31.402 L224.5,122.983 L285.995,31.402 L282.985,142.371 L385.495,105.159 L319.13,193.13 L423.5,224.5 L319.13,255.871 L385.494,343.841 L282.984,306.629 L285.994,417.598 L224.5,326.017 z'
  ]
}

library.add(faSplat);
const faCustomIcon: IconDefinition = {
  prefix: 'fac' as IconPrefix,
  iconName: 'customIcon' as IconName,
  icon: [
    512, 512,
    [],
    '',
    'M43.2,57.3l112,96c12,10.3,29.7,10.3,41.7,0l89.5-76.7l84.3,84.4H352c-17.7,0-32,14.3-32,32s14.3,32,32,32h96 c8.8,0,16.8-3.6,22.6-9.3l0.1-0.1c3-3.1,5.3-6.6,6.9-10.3c1.6-3.7,2.4-7.8,2.4-12.2V193V97c0-17.7-14.3-32-32-32s-32,14.3-32,32 v18.7L310.6,10.4c-11.8-11.8-30.8-12.6-43.5-1.7L176,86.9L84.8,8.7C71.4-2.8,51.2-1.2,39.7,12.2C28.2,25.6,29.8,45.8,43.2,57.3z M464,256H48c-26.5,0-48,21.5-48,48v160c0,26.5,21.5,48,48,48h416c26.5,0,48-21.5,48-48V304C512,277.5,490.5,256,464,256z M48,464v-48c26.5,0,48,21.5,48,48H48z M48,352v-48h48C96,330.5,74.5,352,48,352z M256,448c-35.3,0-64-28.7-64-64 c0-35.3,28.7-64,64-64c35.3,0,64,28.7,64,64C320,419.3,291.3,448,256,448z M464,464h-48c0-26.5,21.5-48,48-48V464z M464,352 c-26.5,0-48-21.5-48-48h48V352z'
  ]
};

library.add(faCustomIcon);
dom.watch();


const modals = {
  prompt: PromptModal
  /* ...other modals */
};
declare module '@mantine/modals' {
  export interface MantineModalsOverride {
    modals: typeof modals;
  }
}


i18n
  .use(initReactI18next)
  .init({
    resources: {
      en: { translation: en },
      dk: { translation: dk },
    },
    lng: "en",
    fallbackLng: "en",
    interpolation: { escapeValue: false }
  });

// Create a Backend Client
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
    },
  },
})

function App() {

  return (
    <QueryClientProvider client={queryClient}>
      <ModalsProvider

        modals={modals}
        modalProps={{
          centered: true,
          onClose() { },
        }}>
        <AppContextProvider>
          <WarframeMarketContextProvider>
            <StockContextProvider>
              <LiveScraperContextProvider>
                <AppRoutes />
              </LiveScraperContextProvider>
            </StockContextProvider>
          </WarframeMarketContextProvider>
        </AppContextProvider>
        {/* <ReactQueryDevtools initialIsOpen={false} /> */}
      </ModalsProvider>
    </QueryClientProvider>
  )
}

export default App
