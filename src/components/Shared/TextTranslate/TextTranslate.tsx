import { Text, MantineSize, TextProps } from "@mantine/core";
import { faCubes, faEnvelope, faHandshake } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon, FontAwesomeIconProps } from "@fortawesome/react-fontawesome";
import { Trans } from "react-i18next";
import { faPlat } from "@icons";

export type TextTranslateProps = TextProps & {
  i18nKey: string;
  color?: string;
  size?: MantineSize | (string & {});
  components?: { [key: string]: React.ReactNode };
  content?: React.ReactNode;
  iconProps?: FontAwesomeIconProps;
  values?: { [key: string]: number | string };
};
export function TextTranslate(prop: TextTranslateProps) {
  const { size, color, i18nKey, values, components, content, iconProps, ...textProps } = prop;

  return (
    <Text size={size ? size : "sm"} c={color ? color : "gray.6"} {...textProps}>
      <Trans
        i18nKey={i18nKey}
        values={values || {}}
        components={{
          ...components,
          blue: <Text {...textProps} component="span" size={size ? size : "sm"} c="blue.3" />,
          red: <Text {...textProps} component="span" size={size ? size : "sm"} c="red.3" />,
          green: <Text {...textProps} component="span" size={size ? size : "sm"} c="green.3" />,
          yellow: <Text {...textProps} component="span" size={size ? size : "sm"} c="yellow.3" />,
          orange: <Text {...textProps} component="span" size={size ? size : "sm"} c="orange.3" />,
          purple: <Text {...textProps} component="span" size={size ? size : "sm"} c="purple.3" />,
          pink: <Text {...textProps} component="span" size={size ? size : "sm"} c="pink.3" />,
          gray: <Text {...textProps} component="span" size={size ? size : "sm"} c="gray.3" />,
          white: <Text {...textProps} component="span" size={size ? size : "sm"} c="white" />,
          violet: <Text {...textProps} component="span" size={size ? size : "sm"} c="violet.3" />,
          cyan: <Text {...textProps} component="span" size={size ? size : "sm"} c="cyan.3" />,
          brown: <Text {...textProps} component="span" size={size ? size : "sm"} c="brown.3" />,
          lime: <Text {...textProps} component="span" size={size ? size : "sm"} c="lime.3" />,
          teal: <Text {...textProps} component="span" size={size ? size : "sm"} c="teal.3" />,
          dark: <Text {...textProps} component="span" size={size ? size : "sm"} c="dark.3" />,
          dark_red: <Text {...textProps} component="span" size={size ? size : "sm"} c="red.9" />,
          qty: <FontAwesomeIcon {...iconProps} icon={faCubes} />,
          mail: <FontAwesomeIcon {...iconProps} icon={faEnvelope} />,
          plat: <FontAwesomeIcon {...iconProps} icon={faPlat} />,
          trade: <FontAwesomeIcon {...iconProps} icon={faHandshake} />,
          // credits: <Image src={"/imgs/credits.png"} width={16} height={16} />,
        }}
      />
      {content}
    </Text>
  );
}
