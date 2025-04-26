import { Box } from "@mantine/core";
import { RivenAttribute } from "$types";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import classes from "./RivenAttribute.module.css";
import { useEffect, useState } from "react";
import { TextTranslate } from "@components/TextTranslate";
import { useTranslateComponent } from "@hooks/useTranslate.hook";

export type RivenAttributeComProps = {
  value: RivenAttribute;
};
export function RivenAttributeCom({ value }: RivenAttributeComProps) {
  const [nameMap, setNameMap] = useState<{ [key: string]: string }>({});

  // Fetch data from rust side
  const { data } = useQuery({
    queryKey: ["cache_riven_attributes"],
    queryFn: () => api.cache.getRivenAttributes(),
    enabled: !value.effect,
  });

  // Set name map
  useEffect(() => {
    if (data) {
      const map: { [key: string]: string } = {};
      data.forEach((item) => {
        map[item.url_name] = item.effect;
      });
      setNameMap(map);
    }
  }, [data]);
  // Translate general
  const useTranslateRivenAttribute = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`riven_attribute.${key}`, { ...context }, i18Key);

  return (
    <Box data-positive={value.positive} className={classes.root}>
      <TextTranslate
        color={value.positive ? "green.9" : "red.7"}
        i18nKey={useTranslateRivenAttribute("effect", undefined, true)}
        values={{
          name: nameMap[value.url_name] || value.effect || value.url_name,
          value: value.value,
        }}
      />
    </Box>
  );
}
