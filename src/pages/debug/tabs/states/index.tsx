import { Accordion, Button, Container, Grid, Group, JsonInput, Text, Title } from "@mantine/core";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { useAppContext } from "@contexts/app.context";
import { useAuthContext } from "@contexts/auth.context";
import { useLiveScraperContext } from "@contexts/liveScraper.context";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { useEffect, useState } from "react";
import { DataTable } from "mantine-datatable";
interface StatesPanelProps {}
export const StatesPanel = ({}: StatesPanelProps) => {
  const { app_info, app_error, alerts, settings } = useAppContext();
  const { user } = useAuthContext();

  // Translate general
  const useTranslateTabLogging = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`debug.tabs.states.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabLogging(`accordions.${key}`, { ...context }, i18Key);

  // State's
  const [tauriEvents, setTauriEvents] = useState<Array<{ event: string; count: number }>>([]);

  // Queries
  const { data: wfmState, refetch: refetchWfmState } = useQuery({
    queryKey: ["get_wfm_state"],
    queryFn: () => api.debug.get_wfm_state(),
    retry: false,
  });

  useEffect(() => {
    setTauriEvents(api.events.listener.getInfo());
  }, []);

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
      <Accordion.Item value="live_scraper">
        <Accordion.Control>{useTranslateDataGridColumns("live_scraper")}</Accordion.Control>
        <Accordion.Panel>
          <JsonInput
            value={JSON.stringify(useLiveScraperContext(), null, 2)}
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
      <Accordion.Item value="wfm_state">
        <Accordion.Control>{useTranslateDataGridColumns("wfm_state")}</Accordion.Control>
        <Accordion.Panel>
          <Group>
            <Button mb={"md"} onClick={() => refetchWfmState()}>
              Refetch
            </Button>
            <Text>Order Limit: {wfmState?.order_limit || "N/A"}</Text>
          </Group>
          <Grid>
            <Grid.Col span={6}>
              <Title order={3} mb={"md"}>
                Buy Orders: {wfmState?.user_orders.buy_orders.length || 0}
              </Title>
              <JsonInput
                value={JSON.stringify(wfmState?.user_orders.buy_orders || {}, null, 2)}
                validationError="Invalid JSON"
                formatOnBlur
                autosize
                minRows={10}
                maxRows={20}
                readOnly
                style={{ width: "100%" }}
              />
            </Grid.Col>
            <Grid.Col span={6}>
              <Title order={3} mb={"md"}>
                Sell Orders: {wfmState?.user_orders.sell_orders.length || 0}
              </Title>
              <JsonInput
                value={JSON.stringify(wfmState?.user_orders.sell_orders || {}, null, 2)}
                validationError="Invalid JSON"
                formatOnBlur
                autosize
                minRows={10}
                maxRows={20}
                readOnly
                style={{ width: "100%" }}
              />
            </Grid.Col>
          </Grid>
        </Accordion.Panel>
      </Accordion.Item>
      <Accordion.Item value="tauri_events">
        <Accordion.Control>{useTranslateDataGridColumns("tauri_events")}</Accordion.Control>
        <Accordion.Panel>
          <DataTable
            records={tauriEvents}
            idAccessor={"event"}
            columns={[
              { accessor: "event", title: "Event" },
              { accessor: "count", title: "Listener Count" },
            ]}
          />
        </Accordion.Panel>
      </Accordion.Item>
    </Accordion>
  );
};
