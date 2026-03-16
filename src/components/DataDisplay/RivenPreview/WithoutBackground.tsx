import { memo } from "react";
import { alpha, Card, Image, Group, Text, Center } from "@mantine/core";
import classes from "./RivenPreview.module.css";
import { upperFirst, useHover } from "@mantine/hooks";
import { RivenAttribute } from "../RivenAttribute";
import { ItemRiven } from "$types";
import { TextTranslate, TextTranslateProps } from "@components/Shared/TextTranslate";
import { useTranslateComponent } from "@hooks/useTranslate.hook";

interface Properties {
  grade?: string;
  challenge_description_with_complication?: string;
}

export type RivenWithoutBackgroundProps = {
  value: ItemRiven<Properties, {}>;
  setDefaultHeaderCenterAs?: "headerLeft" | "headerRight" | "footerLeft" | "footerCenter" | "footerRight" | "disable";
  headerLeft?: TextTranslateProps;
  headerCenter?: TextTranslateProps;
  headerRight?: TextTranslateProps;
  footerLeft?: TextTranslateProps;
  footerCenter?: TextTranslateProps;
  footerRight?: TextTranslateProps;
};
const size = 35;
const grades: Record<string, React.ReactNode> = {
  perfect: <Image className={classes.gradeImage} src="/grades/gradePerfect.png" h={size} w="auto" fit="contain" />,
  good: <Image className={classes.gradeImage} src="/grades/gradeGreen.png" h={size} w="auto" fit="contain" />,
  has_potential: <Image className={classes.gradeImage} src="/grades/gradeYellow.png" h={size} w="auto" fit="contain" />,
  bad: <Image className={classes.gradeImage} src="/grades/gradeRed.png" h={size} w="auto" fit="contain" />,
  unknown: <Image className={classes.gradeImage} src="/question.png" h={size} w="auto" fit="contain" />,
};
export const WithoutBackground = memo(function WithoutBackground({
  value,
  setDefaultHeaderCenterAs,
  footerLeft,
  footerRight,
  footerCenter,
  headerLeft,
  headerCenter,
  headerRight,
}: RivenWithoutBackgroundProps) {
  // State
  const { ref } = useHover();

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`riven_preview.without_background.${key}`, { ...context }, i18Key);

  const RenderDefaultHeaderCenter = () => (
    <TextTranslate
      i18nKey={useTranslate("riven_name", undefined, true)}
      fz={"lg"}
      fw={700}
      values={{
        name: value.name,
        mod_name: upperFirst(value.mod_name),
      }}
    />
  );

  if (!value) return <>...</>;
  const GetProperty = <T extends keyof Properties>(key: T) => {
    return value[key as keyof typeof value] ?? value.properties?.[key];
  };
  return (
    <Card radius="md" ref={ref} pos={"relative"} h={165}>
      <Card.Section bg={alpha("var(--mantine-color-dark-7)", 0.7)} p={3} className={classes.headerSection}>
        <Group justify="space-between" align="center" wrap="nowrap" style={{ overflow: "hidden" }}>
          <div style={{ flex: "0 1 auto", display: "flex", justifyContent: "flex-start", minWidth: 0 }}>
            {headerLeft && <TextTranslate {...headerLeft} />}
            {setDefaultHeaderCenterAs === "headerLeft" && !headerLeft && <RenderDefaultHeaderCenter />}
          </div>
          <div style={{ flex: "1 1 auto", display: "flex", justifyContent: "center", minWidth: 0 }}>
            {!setDefaultHeaderCenterAs && !headerCenter && <RenderDefaultHeaderCenter />}
            {headerCenter && <TextTranslate {...headerCenter} />}
          </div>
          <div style={{ flex: "0 1 auto", display: "flex", justifyContent: "flex-end", minWidth: 0 }}>
            {headerRight && <TextTranslate {...headerRight} />}
            {setDefaultHeaderCenterAs === "headerRight" && !headerRight && <RenderDefaultHeaderCenter />}
          </div>
        </Group>
        {GetProperty("grade") && grades[GetProperty("grade") as string]}
      </Card.Section>
      <Card.Section className={classes.attributesSection}>
        {GetProperty("challenge_description_with_complication") && (
          <Center>
            <Text>{GetProperty("challenge_description_with_complication") as string}</Text>
          </Center>
        )}
        {value.attributes.map((attr) => (
          <RivenAttribute i18nKey="full" key={attr.url_name} groupProps={{ p: 1 }} value={attr} hideDetails centered hideGrade />
        ))}
      </Card.Section>
      <Card.Section bg={alpha("var(--mantine-color-dark-7)", 0.7)} p={3} className={classes.footerSection}>
        <Group justify="space-between" align="center" wrap="nowrap">
          <div style={{ flex: 1, display: "flex", justifyContent: "flex-start" }}>
            {footerLeft && <TextTranslate {...footerLeft} values={{ ...footerLeft?.values, rank: value.sub_type?.rank || 0 }} />}
            {setDefaultHeaderCenterAs === "footerLeft" && !footerLeft && <RenderDefaultHeaderCenter />}
          </div>
          <div style={{ flex: 1, display: "flex", justifyContent: "center" }}>
            {footerCenter && <TextTranslate {...footerCenter} />}
            {setDefaultHeaderCenterAs === "footerCenter" && !footerCenter && <RenderDefaultHeaderCenter />}
          </div>
          <div style={{ flex: 1, display: "flex", justifyContent: "flex-end" }}>
            {footerRight && <TextTranslate {...footerRight} values={{ ...footerRight?.values, mastery: value.mastery_rank }} />}
            {setDefaultHeaderCenterAs === "footerRight" && !footerRight && <RenderDefaultHeaderCenter />}
          </div>
        </Group>
      </Card.Section>
    </Card>
  );
});
