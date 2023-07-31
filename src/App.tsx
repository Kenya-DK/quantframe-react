import './App.css'
import { AuthContextProvider } from './contexts/auth.context'
import AppRoutes from './layouts/routes'
import i18n from "i18next";
import { en } from './lang/en'
import { dk } from './lang/dk'
import { initReactI18next } from "react-i18next";
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import { ModalsProvider } from '@mantine/modals';
import { createStyles } from '@mantine/core';
import { PromptModal } from './components/modals/prompt.modal';
import { settings, user } from './store';
// Create a client
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
    },
  },
})
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

// @ts-ignore
window.debug = async () => {
  const config = structuredClone(await settings.get())
  const currentUser = structuredClone(await user.get())
  // @ts-ignore
  delete config.user_password
  // @ts-ignore
  delete config.access_token

  console.group('Debug')
  console.log(`pathname: ${window.location.pathname}`)
  console.log(`settings: ${JSON.stringify(config, null, 2)}`)
  // console.log('cache:', cache.items)
  console.log('user', currentUser)

  console.groupEnd()
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
        <AuthContextProvider>
          <AppRoutes />
        </AuthContextProvider>
        <ReactQueryDevtools initialIsOpen={false} />
      </ModalsProvider>
    </QueryClientProvider>
  )
}

export default App
