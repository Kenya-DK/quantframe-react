import { memo } from "react";
import { PaperProps } from "@mantine/core";
import { ItemRiven } from "$types/index";
import { WithBackground } from "./WithBackground";
import { WithoutBackground } from "./WithoutBackground";
import { TextTranslateProps } from "@components/Shared/TextTranslate";

export type RivenPreviewProps = {
  value: ItemRiven;
  compact?: boolean;
  type: "withBackground" | "withoutBackground";
  paperProps?: PaperProps;
  headerLeft?: TextTranslateProps;
  headerCenter?: TextTranslateProps;
  setDefaultHeaderCenterAs?: "headerLeft" | "headerRight" | "footerLeft" | "footerCenter" | "footerRight" | "disable";
  headerRight?: TextTranslateProps;
  footerLeft?: TextTranslateProps;
  footerCenter?: TextTranslateProps;
  footerRight?: TextTranslateProps;
};

export const RivenPreview = memo(function RivenPreview(props: RivenPreviewProps) {
  // State
  return (
    <>
      {props.type == "withBackground" && <WithBackground {...props} />}
      {props.type == "withoutBackground" && <WithoutBackground {...props} />}
    </>
  );
});
