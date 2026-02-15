import { memo } from "react";
import { alpha, Card, Image, Grid } from "@mantine/core";
import classes from "./RivenPreview.module.css";
import { upperFirst, useHover } from "@mantine/hooks";
import { RivenAttribute } from "../RivenAttribute";
import { RivenProps } from "./RivenPreview";
import { TextTranslate, TextTranslateProps } from "@components/Shared/TextTranslate";
import { useTranslateComponent } from "@hooks/useTranslate.hook";

export type RivenWithoutBackgroundProps = {
  value: RivenProps;
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
  value: riven,
  footerLeft,
  footerRight,
  footerCenter,
}: RivenWithoutBackgroundProps) {
  // State
  const { ref } = useHover();

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`riven_preview.without_background.${key}`, { ...context }, i18Key);

  return (
    <Card radius="md" ref={ref} pos={"relative"}>
      <Card.Section bg={alpha("var(--mantine-color-dark-7)", 0.7)} p={3} className={classes.headerSection}>
        <TextTranslate
          i18nKey={useTranslate("riven_name", undefined, true)}
          values={{
            name: riven.weapon.name,
            mod_name: upperFirst(riven.modName),
          }}
          textProps={{ ta: "center", fz: "lg", fw: 700 }}
        />
        {grades[riven.grade || "unknown"]}
      </Card.Section>
      <Card.Section className={classes.attributesSection}>
        {riven.attributes.map((attr) => (
          <RivenAttribute i18nKey="full" key={attr.url_name} groupProps={{ p: 1 }} value={attr} hideDetails centered hideGrade />
        ))}
      </Card.Section>
      <Card.Section bg={alpha("var(--mantine-color-dark-7)", 0.7)} p={3} className={classes.footerSection}>
        <Grid>
          <Grid.Col span={4}>
            <TextTranslate
              i18nKey={useTranslate("footer_left", undefined, true)}
              textProps={{ fz: "lg", fw: 700 }}
              {...footerLeft}
              values={{ ...footerLeft?.values, rank: riven.rank }}
            />
          </Grid.Col>
          <Grid.Col span={4}>{footerCenter && <TextTranslate {...footerCenter} textProps={{ ta: "center", fz: "lg", fw: 700 }} />}</Grid.Col>
          <Grid.Col span={4}>
            <TextTranslate
              i18nKey={useTranslate("footer_right", undefined, true)}
              textProps={{ ta: "right", fz: "lg", fw: 700 }}
              {...footerRight}
              values={{ ...footerRight?.values, mastery: riven.mastery }}
            />
          </Grid.Col>
        </Grid>
      </Card.Section>
    </Card>
  );
});
