import ReactDOM from "react-dom/client";
import "@mantine/core/styles.css";
import "@mantine/notifications/styles.css";
import "mantine-datatable/styles.layer.css";
import "@mantine/tiptap/styles.css";
import "@mantine/dates/styles.css";
import "./global.css";
import App from "./App.tsx";
import { StrictMode } from "react";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>
);
