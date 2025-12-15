import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ModalsProvider } from "@mantine/modals";
import i18n from "i18next";
import classes from "./modals.module.css";
import { initReactI18next } from "react-i18next";
import { DatesProvider } from "@mantine/dates";
import { library, dom } from "@fortawesome/fontawesome-svg-core";
import { AppRoutes } from "@components/Layouts/Routes";
import { AppContextProvider } from "@contexts/app.context";
import faMoneyBillTrendDown from "@icons/faMoneyBillTrendDown";
import faTradingAnalytics from "@icons/faTradingAnalytics";
import faAmberStar from "@icons/faAmberStar";
import faCyanStar from "@icons/faCyanStar";
import faInfinity from "@icons/faInfinity";
import faPlat from "@icons/faPlat";
import faPolarity from "@icons/faPolarity";
import faPolarityAny from "@icons/faPolarityAny";
import faPolarityZenuri from "@icons/faPolarityZenuri";
import faPolarityUnairu from "@icons/faPolarityUnairu";
import faPolarityUmbra from "@icons/faPolarityUmbra";
import faPolarityPenjaga from "@icons/faPolarityPenjaga";
import faPolarityNaramon from "@icons/faPolarityNaramon";
import faPolarityMadurai from "@icons/faPolarityMadurai";
import faWebHook from "@icons/faWebHook";
import faPolarityAura from "@icons/faPolarityAura";
import faPolarityVazarin from "@icons/faPolarityVazarin";
import { useEffect, useState } from "react";
import api from "./api";
import { PromptModal } from "@components/Modals/Prompt";
import { PatreonModal } from "@components/Modals/PatreonModal/indexx";
import { MantineProvider } from "@mantine/core";
import { Notifications } from "@mantine/notifications";
import { ThemeProvider, useTheme } from "./contexts/theme.context";
library.add(faMoneyBillTrendDown);
library.add(faTradingAnalytics);
library.add(faAmberStar);
library.add(faCyanStar);
library.add(faInfinity);
library.add(faPlat);
library.add(faPolarity);
library.add(faPolarityAny);
library.add(faPolarityZenuri);
library.add(faPolarityUnairu);
library.add(faPolarityUmbra);
library.add(faPolarityPenjaga);
library.add(faPolarityNaramon);
library.add(faPolarityMadurai);
library.add(faPolarityAura);
library.add(faPolarityVazarin);
library.add(faWebHook);
dom.watch();

export async function initializeI18n() {
  const response = await fetch("/lang/en.json");
  const translations = await response.json();
  await i18n.use(initReactI18next).init({
    resources: {
      en: { translation: translations },
    },
    lng: "en",
    fallbackLng: "en",
    interpolation: { escapeValue: false },
  });

  console.log("Loaded default language from filesystem:", "resources/lang/en.json");
}
// Create a Backend Client
export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
    },
  },
});
const modals = {
  prompt: PromptModal,
  patreon: PatreonModal,
  /* ...other modals */
};
export interface MantineModalsOverride {
  modals: typeof modals;
}
// AppContent component that uses the theme context
function AppContent() {
  const { theme, resolver } = useTheme();
  const [ready, setReady] = useState(false);

  useEffect(() => {
    initializeI18n().then(() => setReady(true));
  }, []);

  if (!ready) return null;

  return (
    <MantineProvider defaultColorScheme="dark" theme={theme} cssVariablesResolver={resolver}>
      <Notifications position="bottom-right" />
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
            <AppRoutes />
          </AppContextProvider>
        </DatesProvider>
        {/* <ReactQueryDevtools initialIsOpen={false} /> */}
      </ModalsProvider>
    </MantineProvider>
  );
}

function App() {
  useEffect(() => {
    window.onclick = async () => await api.analytics.setLastUserActivity();
  }, []);
  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider>
        <AppContent />
      </ThemeProvider>
    </QueryClientProvider>
  );
}

export default App;
