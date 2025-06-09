import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ModalsProvider } from "@mantine/modals";
import i18n from "i18next";
import classes from "./modals.module.css";
import { initReactI18next } from "react-i18next";
import { en } from "./lang/en";
import { dk } from "./lang/dk";
import { DatesProvider } from "@mantine/dates";
import { library, dom } from "@fortawesome/fontawesome-svg-core";
import { AppRoutes } from "@components/Layouts/Routes";
import { PromptModal } from "@components/Modals/Prompt";
import { AppContextProvider } from "@contexts/app.context";
import { LiveScraperContextProvider } from "@contexts/liveScraper.context";
import { WarframeMarketContextProvider } from "@contexts/warframeMarket.context";
import { useEffect } from "react";
import api from "./api";
import faMoneyBillTrendDown from "@icons/faMoneyBillTrendDown";
import faTradingAnalytics from "@icons/faTradingAnalytics";
library.add(faMoneyBillTrendDown);
library.add(faTradingAnalytics);
dom.watch();
const modals = {
  prompt: PromptModal,
  /* ...other modals */
};
export interface MantineModalsOverride {
  modals: typeof modals;
}

i18n.use(initReactI18next).init({
  resources: {
    en: { translation: en },
    dk: { translation: dk },
  },
  lng: "en",
  fallbackLng: "en",
  interpolation: { escapeValue: false },
});

// Create a Backend Client
export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
    },
  },
});

function App() {
  useEffect(() => {
    window.onclick = async () => {
      await api.analytics.setLastUserActivity();
    };
  }, []);

  return (
    <QueryClientProvider client={queryClient}>
      <ModalsProvider
        modals={modals}
        modalProps={{
          centered: true,
          classNames: classes,
          onClose() {},
        }}
      >
        <DatesProvider settings={{ locale: "en" }}>
          <AppContextProvider>
            <WarframeMarketContextProvider>
              <LiveScraperContextProvider>
                <AppRoutes />
              </LiveScraperContextProvider>
            </WarframeMarketContextProvider>
          </AppContextProvider>
        </DatesProvider>
        {/* <ReactQueryDevtools initialIsOpen={false} /> */}
      </ModalsProvider>
    </QueryClientProvider>
  );
}

export default App;
