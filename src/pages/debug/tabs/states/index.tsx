import { Accordion, JsonInput } from "@mantine/core";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { useAppContext } from "@contexts/app.context";
import { useAuthContext } from "@contexts/auth.context";
interface StatesPanelProps {}
export const StatesPanel = ({}: StatesPanelProps) => {
  const { app_info, app_error, alerts, settings } = useAppContext();
  const { user } = useAuthContext();

  // Translate general
  const useTranslateTabLogging = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`debug.tabs.states.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabLogging(`accordions.${key}`, { ...context }, i18Key);

  return (
    <Accordion>
      <Accordion.Item value="app_info">
        <Accordion.Control>{useTranslateDataGridColumns("app_info")}</Accordion.Control>
        <Accordion.Panel>
          <JsonInput
            value={JSON.stringify(app_info, null, 2)}
            validationError="Invalid JSON"
            formatOnBlur
            autosize
            minRows={10}
            maxRows={20}
            readOnly
            style={{ width: "100%" }}
          />
        </Accordion.Panel>
      </Accordion.Item>
      <Accordion.Item value="app_error">
        <Accordion.Control>{useTranslateDataGridColumns("app_error")}</Accordion.Control>
        <Accordion.Panel>
          <JsonInput
            value={JSON.stringify(app_error, null, 2)}
            validationError="Invalid JSON"
            formatOnBlur
            autosize
            minRows={10}
            maxRows={20}
            readOnly
            style={{ width: "100%" }}
          />
        </Accordion.Panel>
      </Accordion.Item>
      <Accordion.Item value="alerts">
        <Accordion.Control>{useTranslateDataGridColumns("alerts")}</Accordion.Control>
        <Accordion.Panel>
          <JsonInput
            value={JSON.stringify(alerts, null, 2)}
            validationError="Invalid JSON"
            formatOnBlur
            autosize
            minRows={10}
            maxRows={20}
            readOnly
            style={{ width: "100%" }}
          />
        </Accordion.Panel>
      </Accordion.Item>

      <Accordion.Item value="settings">
        <Accordion.Control>{useTranslateDataGridColumns("settings")}</Accordion.Control>
        <Accordion.Panel>
          <JsonInput
            value={JSON.stringify(settings, null, 2)}
            validationError="Invalid JSON"
            formatOnBlur
            autosize
            minRows={10}
            maxRows={20}
            readOnly
            style={{ width: "100%" }}
          />
        </Accordion.Panel>
      </Accordion.Item>

      <Accordion.Item value="user">
        <Accordion.Control>{useTranslateDataGridColumns("user")}</Accordion.Control>
        <Accordion.Panel>
          <JsonInput
            value={JSON.stringify(user, null, 2)}
            validationError="Invalid JSON"
            formatOnBlur
            autosize
            minRows={10}
            maxRows={20}
            readOnly
            style={{ width: "100%" }}
          />
        </Accordion.Panel>
      </Accordion.Item>
    </Accordion>
  );
};
