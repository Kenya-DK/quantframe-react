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
  const [searchIndexes, setSearchIndexes] = useState<number[]>([]);
  const [currentMatch, setCurrentMatch] = useState(0);
  // Query's
  const { data: lines, refetch: refetchLines } = useQuery({
    queryKey: ["log_parser_lines"],
    queryFn: () => api.log_parser.getLogEELines(),
  });
  const { data: ReadDate, refetch: refetchReadDate } = useQuery({
    queryKey: ["last_read_date"],
    queryFn: () => api.log_parser.getLastReadDate(),
  });

  // Method's
  const UpdateIndexes = (text: string) => {
    let indexes: number[] = [];
    indexes = lines?.map((line, idx) => (line.toLowerCase().includes(text.toLowerCase()) ? idx : -1)).filter((idx) => idx !== -1) || [];
    setSearchIndexes(indexes);
    setCurrentMatch(0);
  };

  // Update Rows and Total Pages when lines or pageSize changes
  useEffect(() => {
    let filteredRecords = lines || [];
    if (!lines) return;

    // Update total pages
    setTotalPages(Math.ceil(filteredRecords.length / pageSize));

    setRows(paginate(filteredRecords, page, pageSize));
  }, [lines, pageSize, page]);

  return (
    <Box>
      <SearchField
        value={query}
        onSearch={(text) => UpdateIndexes(text)}
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
      <Code block className={`${classes.log_parser} ${useHasAlert() ? classes.alert : ""}`}>
        {rows.length > 0
          ? rows.map((line, idx) => {
              const globalIdx = (page - 1) * pageSize + idx;
              let isCurrentMatch = false;
              if (searchIndexes.length > 0 && searchIndexes[currentMatch] === globalIdx) {
                isCurrentMatch = true;
              }
              if (query && line.toLowerCase().includes(query.toLowerCase())) {
                // Highlight all matches in the line, and extra highlight for the selected index
                const regex = new RegExp(`(${query.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")})`, "gi");
                let lastIndex = 0;
                const parts = [];
                let match;
                let matchIdx = 0;
                while ((match = regex.exec(line)) !== null) {
                  if (match.index > lastIndex) {
                    parts.push(<span key={lastIndex}>{line.slice(lastIndex, match.index)}</span>);
                  }
                  // If this is the selected match on the selected line, use a stronger highlight
                  const highlightStyle =
                    isCurrentMatch && matchIdx === 0
                      ? { background: "#ffd43b", color: "#000", padding: 0, border: "1px solid #ffb700" }
                      : { background: "#ffe066", color: "#000", padding: 0 };
                  parts.push(
                    <mark key={match.index} style={highlightStyle}>
                      {match[0]}
                    </mark>
                  );
                  lastIndex = match.index + match[0].length;
                  matchIdx++;
                }
                if (lastIndex < line.length) {
                  parts.push(<span key={lastIndex}>{line.slice(lastIndex)}</span>);
                }
                return (
                  <span key={idx} style={{ display: "block", outline: isCurrentMatch ? "2px solid #ffd43b" : undefined, borderRadius: 4 }}>
                    {parts}
                  </span>
                );
              } else {
                return (
                  <span key={idx} style={{ display: "block" }}>
                    {line}
                  </span>
                );
              }
            })
          : "Nothing"}
      </Code>
      <Divider mt={"md"} />
      <Group grow mt={"md"}>
        <Group>
          {(pageSize * page > (lines?.length || 0) ? lines?.length || "0" : pageSize * page) || "0"}/{lines?.length || "0"} records
        </Group>
        <Group justify="flex-end">
          <Pagination value={page} onChange={setPage} total={totalPages} />
        </Group>
        <Group justify="flex-end">
          <Text size="sm" c="dimmed">
            {searchIndexes.length > 0 ? `Match ${currentMatch + 1} of ${searchIndexes.length}` : "No matches found"}
          </Text>
          <Button
            size="xs"
            disabled={searchIndexes.length === 0}
            onClick={() => {
              if (currentMatch + 1 < searchIndexes.length) {
                setCurrentMatch(currentMatch + 1);
                setPage(Math.ceil((searchIndexes[currentMatch + 1] + 1) / pageSize));
              }
            }}
          >
            Next Match
          </Button>
          <Button
            size="xs"
            disabled={searchIndexes.length === 0}
            onClick={() => {
              if (currentMatch - 1 >= 0) {
                setCurrentMatch(currentMatch - 1);
                setPage(Math.ceil((searchIndexes[currentMatch - 1] + 1) / pageSize));
              }
            }}
          >
            Previous Match
          </Button>
          <Button
            size="xs"
            disabled={searchIndexes.length === 0}
            onClick={() => {
              if (searchIndexes.length > 0) {
                setPage(Math.ceil((searchIndexes[currentMatch] + 1) / pageSize));
              }
            }}
          >
            Go to Match {page}
          </Button>
        </Group>
      </Group>
    </Box>
  );
};
