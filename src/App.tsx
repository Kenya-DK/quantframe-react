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
import faPolarityAura from "@icons/faPolarityAura";
import faPolarityVazarin from "@icons/faPolarityVazarin";
import { useEffect } from "react";
import api from "./api";
import { PromptModal } from "./components/Modals/Prompt";
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
dom.watch();
import { check } from "@tauri-apps/plugin-updater";
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
const modals = {
  prompt: PromptModal,
  /* ...other modals */
};
export interface MantineModalsOverride {
  modals: typeof modals;
}
const update = await check();
console.log("Update check result:", update);
if (update) {
  console.log(`found update ${update.version} from ${update.date} with notes ${update.body}`);
  // return;
  // // alternatively we could also call update.download() and update.install() separately
  // await update.downloadAndInstall((event) => {
  //   switch (event.event) {
  //     case "Started":
  //       contentLength = event.data.contentLength || 0;
  //       console.log(`started downloading ${event.data.contentLength} bytes`);
  //       break;
  //     case "Progress":
  //       downloaded += event.data.chunkLength;
  //       console.log(`downloaded ${downloaded} from ${contentLength}`);
  //       break;
  //     case "Finished":
  //       console.log("download finished");
  //       break;
  //   }
  // });

  // console.log("update installed");
  // await relaunch();
}
function App() {
  useEffect(() => {
    window.onclick = async () => await api.analytics.setLastUserActivity();
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
            <AppRoutes />
          </AppContextProvider>
        </DatesProvider>
        {/* <ReactQueryDevtools initialIsOpen={false} /> */}
      </ModalsProvider>
    </QueryClientProvider>
  );
}

export default App;
