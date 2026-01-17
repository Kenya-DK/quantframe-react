import { Tooltip, ActionIcon, MantineColor, ActionIconProps } from "@mantine/core";
import { FontAwesomeIcon, FontAwesomeIconProps } from "@fortawesome/react-fontawesome";
import { IconProp } from "@fortawesome/fontawesome-svg-core";
export type ActionWithTooltipProps = {
  color?: MantineColor;
  tooltip: string;
  width?: number;
  icon: IconProp;
  loading?: boolean;
  iconProps?: Omit<FontAwesomeIconProps, "icon">;
  actionProps?: ActionIconProps & { type?: "button" | "submit" | "reset" };
  onClick: (e: React.MouseEvent<HTMLButtonElement>) => void;
};

export function ActionWithTooltip({ width, loading, iconProps, actionProps, tooltip, icon, color, onClick }: ActionWithTooltipProps) {
  return (
    <Tooltip label={tooltip}>
      <ActionIcon w={width} loading={loading} {...actionProps} color={color} variant="filled" onClick={async (e) => onClick(e)}>
        <FontAwesomeIcon {...iconProps} icon={icon} />
      </ActionIcon>
    </Tooltip>
  );
}
