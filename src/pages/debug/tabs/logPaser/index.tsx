import { Box, Divider, Group, Text, Pagination, Alert, Button, Code } from "@mantine/core";
import api from "@api/index";
import { useQuery } from "@tanstack/react-query";
import { SearchField } from "@components/SearchField";
import { useEffect, useState } from "react";
import { paginate } from "@utils/helper";
import { ActionWithTooltip } from "@components/ActionWithTooltip";
import { faBookOpen, faRefresh, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { modals } from "@mantine/modals";
import { open } from "@tauri-apps/plugin-shell";
import classes from "../../Debug.module.css";
import { useHasAlert } from "@hooks/useHasAlert.hook";
interface LogParserPanelProps {}
export const LogParserPanel = ({}: LogParserPanelProps) => {
  const [query, setQuery] = useState<string>("");
  const [page, setPage] = useState(1);
  const [pageSize, _setPageSize] = useState(50);
  const [totalPages, setTotalPages] = useState(0);
  const [rows, setRows] = useState<string[]>([]);
  const { data: lines, refetch: refetchLines } = useQuery({
    queryKey: ["log_parser_lines"],
    queryFn: () => api.log_parser.getLogEELines(),
  });
  const { data: ReadDate, refetch: refetchReadDate } = useQuery({
    queryKey: ["last_read_date"],
    queryFn: () => api.log_parser.getLastReadDate(),
  });
  // Update Database Rows
  useEffect(() => {
    let filteredRecords = lines || [];
    if (!lines) return;

    // Update total pages
    setTotalPages(Math.ceil(filteredRecords.length / pageSize));

    setRows(paginate(filteredRecords, page, pageSize));
  }, [lines, query, pageSize, page]);
  return (
    <Box>
      <SearchField
        value={query}
        onChange={(text) => setQuery(text)}
        rightSectionWidth={115}
        rightSection={
          <Group gap={5}>
            <ActionWithTooltip
              tooltip={"Refresh"}
              icon={faRefresh}
              color={"blue.7"}
              actionProps={{ size: "sm" }}
              iconProps={{ size: "xs" }}
              onClick={async () => {
                await refetchLines();
                await refetchReadDate();
              }}
            />
            <ActionWithTooltip
              tooltip={"Dump Log Cache to Database"}
              icon={faBookOpen}
              color={"violet.7"}
              actionProps={{ size: "sm" }}
              iconProps={{ size: "xs" }}
              onClick={async () => {
                let path = await api.log_parser.dumpLogCache();
                modals.open({
                  title: "Log Cache Dumped",
                  size: "lg",
                  children: (
                    <Box>
                      <Text>Log Cache has been dumped to your Desktop</Text>
                      <Text>At the following path:</Text>
                      <Text fw={700}>{path}</Text>
                      <Text>Please send this file to the developer (Kenya-DK) in a dm</Text>
                      <Alert title="Disclaimer" color="red" withCloseButton>
                        Please note that this file contains sensitive information, please do not share it with anyone other than the developer. (such
                        as your IP address and email address)
                        <Group mt={5}>
                          <Button
                            size="xs"
                            onClick={() => {
                              open("https://wiki.warframe.com/w/EE.log");
                            }}
                          >
                            Read More
                          </Button>
                        </Group>
                      </Alert>
                    </Box>
                  ),
                });
              }}
            />

            <ActionWithTooltip
              tooltip={"Clear Log Cache"}
              icon={faTrashCan}
              color={"red.7"}
              actionProps={{ size: "sm" }}
              iconProps={{ size: "xs" }}
              onClick={async () => {
                modals.openConfirmModal({
                  title: "Delete Log Warframe EE.log Cache",
                  children: (
                    <Text size="sm">
                      Are you sure you want to delete the log cache? This will not delete your database or any other files. This will only delete the
                      log cache file.
                    </Text>
                  ),
                  labels: { confirm: "Yes", cancel: "No" },
                  onConfirm: async () => {
                    await api.log_parser.clearLogCache();
                    await refetchLines();
                    await refetchReadDate();
                  },
                });
              }}
            />
          </Group>
        }
      />
      <Group gap={"md"} mt={"md"} grow>
        <Group>
          <Text>Last Read Date: {ReadDate}</Text>
        </Group>
      </Group>
      <Divider mt={"md"} />
      {/* <ScrollArea className={`${classes.log_parser} ${useHasAlert() ? classes.alert : ""}`}>
        {rows.map((line) => (
          <Box p={5} key={line}>
            <Text>{line}</Text>
            <Divider />
          </Box>
        ))}
      </ScrollArea> */}
      <Code block className={`${classes.log_parser} ${useHasAlert() ? classes.alert : ""}`}>
        {rows.length > 0 ? rows.join("\n") : "Nothing"}
      </Code>
      <Divider mt={"md"} />
      <Group grow mt={"md"}>
        <Group>
          {rows.length || "0"}/{lines?.length || "0"} records
        </Group>
        <Group justify="flex-end">
          <Pagination value={page} onChange={setPage} total={totalPages} />
        </Group>
      </Group>
    </Box>
  );
};
