import { memo } from "react";
import { Box, PaperProps, Stack, Text } from "@mantine/core";
import classes from "./RivenPreview.module.css";
import { useHover } from "@mantine/hooks";
import { RivenAttribute } from "../RivenAttribute";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faArrowsRotate } from "@fortawesome/free-solid-svg-icons";
import { getPolarityIcon } from "@icons";
import { ItemRiven } from "$types";

export type RivenWithBackgroundProps = {
  value: ItemRiven;
  compact?: boolean;
  paperProps?: PaperProps;
};

export const WithBackground = memo(function WithBackground({ paperProps, value: riven, compact }: RivenWithBackgroundProps) {
  // State
  const { ref } = useHover();

  return (
    <Box {...paperProps} data-compact={compact} className={classes.root} ref={ref}>
      <FontAwesomeIcon data-compact={compact} className={classes.polarity} icon={getPolarityIcon(riven.polarity)} />
      <Text data-compact={compact} className={classes.weapon}>
        {riven?.name}
      </Text>
      <Text data-compact={compact} className={classes.mod_name}>
        {riven.mod_name}
      </Text>
      <Stack gap={0} data-compact={compact} className={classes.attributes}>
        {riven.attributes.map((attr) => (
          <RivenAttribute i18nKey="full" key={attr.url_name} groupProps={{ p: 0, gap: 0 }} value={attr} hideDetails centered hideGrade compact />
        ))}
      </Stack>
      <Text data-compact={compact} className={classes.mastery}>
        MR {riven.mastery_rank > 16 ? 16 : riven.mastery_rank}
      </Text>
      {riven.re_rolls > 0 && (
        <Text data-compact={compact} className={classes.reroll}>
          <FontAwesomeIcon icon={faArrowsRotate} />
          <Text component="span" ml={5}>
            {riven.re_rolls}
          </Text>
        </Text>
      )}
      <Box data-compact={compact} className={classes.rank}>
        {Array.from(Array(Math.min(riven.sub_type?.rank ?? 0, 8))).map((_, i) => {
          return (
            <Text key={i} className={classes.circle} size="sm" component="span">
              ●
            </Text>
          );
        })}
      </Box>
    </Box>
  );
});
