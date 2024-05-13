import { Text, MantineSize, MantineStyleProp, TextProps } from '@mantine/core';
import { faCubes, faEnvelope, faHandshake } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { Trans } from 'react-i18next';

export type TextTranslateProps = {
	i18nKey: string;
	color?: string;
	size?: MantineSize | (string & {});
	style?: MantineStyleProp;
	values: { [key: string]: number | string }
	components?: { [key: string]: React.ReactNode }
	content?: React.ReactNode;
	textProps?: TextProps;
}
export function TextTranslate({ textProps, style, size, color, i18nKey, values, components, content }: TextTranslateProps) {

	return (
		<Text {...textProps} style={{ ...style }} size={size ? size : "sm"} c={color ? color : "gray.6"}>
			<Trans
				i18nKey={i18nKey}
				values={values}
				components={
					{
						...components,
						blue: <Text component="span" size={size ? size : "sm"} c="blue.3" />,
						red: <Text component="span" size={size ? size : "sm"} c="red.3" />,
						green: <Text component="span" size={size ? size : "sm"} c="green.3" />,
						yellow: <Text component="span" size={size ? size : "sm"} c="yellow.3" />,
						orange: <Text component="span" size={size ? size : "sm"} c="orange.3" />,
						purple: <Text component="span" size={size ? size : "sm"} c="purple.3" />,
						pink: <Text component="span" size={size ? size : "sm"} c="pink.3" />,
						gray: <Text component="span" size={size ? size : "sm"} c="gray.3" />,
						violet: <Text component="span" size={size ? size : "sm"} c="violet.3" />,
						cyan: <Text component="span" size={size ? size : "sm"} c="cyan.3" />,
						brown: <Text component="span" size={size ? size : "sm"} c="brown.3" />,
						lime: <Text component="span" size={size ? size : "sm"} c="lime.3" />,
						teal: <Text component="span" size={size ? size : "sm"} c="teal.3" />,
						dark: <Text component="span" size={size ? size : "sm"} c="dark.3" />,
						qty: <FontAwesomeIcon icon={faCubes} />,
						mail: <FontAwesomeIcon icon={faEnvelope} />,
						// plat: <SvgIcon svgProp={{ width: 16, height: 16, }} iconType={SvgType.Default} iconName={"plat"} />,
						trade: <FontAwesomeIcon icon={faHandshake} />,
						// credits: <Image src={"/imgs/credits.png"} width={16} height={16} />,
					}
				}
			/>
			{content}
		</Text>);
}