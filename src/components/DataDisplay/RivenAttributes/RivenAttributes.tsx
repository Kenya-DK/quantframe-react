import { ActionIcon, Box, Group, Paper, Tooltip } from "@mantine/core";
import { TauriTypes } from "$types";
import classes from "./RivenAttributes.module.css";
import { RivenAttribute } from "../RivenAttribute/RivenAttribute";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faInfo } from "@fortawesome/free-solid-svg-icons";
export type RivenAttributesProps = {
  tooltip: boolean;
  attributes: TauriTypes.StockRiven["attributes"];
};

export function RivenAttributes({ attributes, tooltip }: RivenAttributesProps) {
  // Functions

  return (
    <Group mt={5} classNames={classes} p={5}>
      {tooltip ? (
        <Tooltip
          withArrow
          openDelay={100}
          closeDelay={100}
          styles={{
            tooltip: { backgroundColor: "transparent", padding: 0, boxShadow: "none" },
            arrow: { backgroundColor: "transparent", borderWidth: 0 },
          }}
          label={
            <Paper withBorder p="xs">
              <Box style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
                {attributes.map((attr, idx) => (
                  <RivenAttribute key={idx} value={attr} />
                ))}
              </Box>
            </Paper>
          }
        >
          <ActionIcon size="sm" variant="outline">
            <FontAwesomeIcon icon={faInfo} />
          </ActionIcon>
        </Tooltip>
      ) : (
        <>
          {attributes.map((attr) => (
            <RivenAttribute key={attr.url_name} value={attr} />
          ))}
        </>
      )}
    </Group>
  );
}
