import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import { MantineProvider } from '@mantine/core';
import { Notifications } from '@mantine/notifications';
import i18n from "i18next";
import { en } from './lang/en'
import { dk } from './lang/dk'
import { initReactI18next } from "react-i18next";
// Create a client
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

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  //<React.StrictMode>
  <MantineProvider
    withGlobalStyles
    withNormalizeCSS
    theme={{
      /** Put your mantine theme override here */
      colorScheme: 'dark',
      colors: {
        // override dark colors to change them for all components
        dark: [
          '#d5d7e0',
          '#acaebf',
          '#8c8fa3',
          '#666980',
          '#4d4f66',
          '#34354a',
          '#2b2c3d',
          '#1d1e30',
          '#0c0d21',
          '#01010a',
        ],
      }
    }}>
    <Notifications position="bottom-right" />
    <App />
  </MantineProvider>
  //</React.StrictMode>,
)