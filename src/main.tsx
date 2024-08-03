import ReactDOM from 'react-dom/client'
import '@mantine/core/styles.css';
import '@mantine/notifications/styles.css';
import 'mantine-datatable/styles.layer.css';
import './global.css'
import App from './App.tsx'
import { MantineProvider, createTheme } from '@mantine/core';
import { Notifications } from '@mantine/notifications';

const theme = createTheme({
  colors: {
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
    ]
  },
});
ReactDOM.createRoot(document.getElementById('root')!).render(

  <MantineProvider defaultColorScheme="dark" theme={theme}>
    <Notifications position="bottom-right" />
    <App />
  </MantineProvider>

)
