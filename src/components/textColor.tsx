import { Trans } from "react-i18next";
import { MantineNumberSize, Sx, Text } from "@mantine/core";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCubes } from "@fortawesome/free-solid-svg-icons";
import SvgIcon, { SvgType } from "./SvgIcon";
interface TextColorProps {
  i18nKey: string;
  color?: string;
  size?: MantineNumberSize;
  sx?: Sx | (Sx | undefined)[];
  values: { [key: string]: number | string }
  conponents?: { [key: string]: React.ReactNode }
}
export const TextColor = ({ sx, size, color, i18nKey, values, conponents }: TextColorProps) => {
  return (
    <Text sx={{ ...sx }} size={size ? size : "sm"} color={color ? color : "gray.6"}>
      <Trans
        i18nKey={i18nKey}
        values={values}
        components={
          {
            ...conponents,
            blue: <Text component="span" size={size ? size : "sm"} color="blue.3" />,
            red: <Text component="span" size={size ? size : "sm"} color="red.3" />,
            green: <Text component="span" size={size ? size : "sm"} color="green.3" />,
            yellow: <Text component="span" size={size ? size : "sm"} color="yellow.3" />,
            orange: <Text component="span" size={size ? size : "sm"} color="orange.3" />,
            purple: <Text component="span" size={size ? size : "sm"} color="purple.3" />,
            pink: <Text component="span" size={size ? size : "sm"} color="pink.3" />,
            gray: <Text component="span" size={size ? size : "sm"} color="gray.3" />,
            violet: <Text component="span" size={size ? size : "sm"} color="violet.3" />,
            cyan: <Text component="span" size={size ? size : "sm"} color="cyan.3" />,
            brown: <Text component="span" size={size ? size : "sm"} color="brown.3" />,
            lime: <Text component="span" size={size ? size : "sm"} color="lime.3" />,
            teal: <Text component="span" size={size ? size : "sm"} color="teal.3" />,
            dark: <Text component="span" size={size ? size : "sm"} color="dark.3" />,
            qty: <FontAwesomeIcon icon={faCubes} />,
            plat: <SvgIcon svgProp={{ width: 16, height: 16, }} iconType={SvgType.Default} iconName={"plat"} />,
          }
        }
      />
    </Text>)
}