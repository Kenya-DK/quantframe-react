import React, { useEffect, useState } from "react";
import { useDynamicSvgImport, useDynamicPolaritySvgImport } from "@hooks/index";

// SvgType is an enum that is used to determine which svg icon to render
export enum SvgType {
  Default = "default",
  Faction = "factions",
  Polarity = "polarity",
}

export type SvgProps = {
  iconName: string;
  iconType: SvgType;
  style?: React.CSSProperties;
  svgProp?: React.SVGProps<SVGSVGElement>;
  selected?: boolean;
}

export function SvgIcon(props: SvgProps) {
  const { iconType } = props;
  return (
    <>
      {iconType == SvgType.Default && (
        <SvgIconDefault {...props} />
      )}
      {iconType == SvgType.Polarity && (
        <SvgIconPolarity {...props} />
      )}
    </>
  )
}

// SvgIconDefault is a component that is used to render the svg icon
function SvgIconDefault(props: SvgProps) {
  const { iconName } = props;
  const dynamicSvg = useDynamicSvgImport(iconName);
  return (
    <SvgCore
      dynamicSvg={dynamicSvg}
      {...props}
    />
  )
}
// SvgIconDefault is a component that is used to render the svg icon
function SvgIconPolarity(props: SvgProps) {
  const { iconName } = props;
  const dynamicSvg = useDynamicPolaritySvgImport(iconName);
  return (
    <SvgCore
      dynamicSvg={dynamicSvg}
      {...props}
    />
  )
}


// SvgCore is a component that is used to render the svg icon
export type SvgCoreProps = {
  dynamicSvg: {
    error: any, loading: boolean, SvgIcon: React.FC<React.SVGProps<SVGElement>> | undefined
  }
  style?: React.CSSProperties;
  svgProp?: React.SVGProps<SVGSVGElement>;
  selected?: boolean;
}
function SvgCore(props: SvgCoreProps) {
  const { svgProp, selected, dynamicSvg } = props;
  const [color, setColor] = useState<string>("gray");

  useEffect(() => {
    if (svgProp?.fill)
      setColor(svgProp.fill);
    else if (selected)
      setColor("#61dafb");
    else
      setColor("gray");
  }, [selected, svgProp]);

  return (
    <>
      {dynamicSvg.error && (
        <div className="rounded-full bg-slate-400 h-8 w-8">{JSON.stringify(dynamicSvg.error)}</div>
      )}
      {dynamicSvg.loading && (
        <div className="rounded-full bg-slate-400 animate-pulse h-8 w-8"></div>
      )}
      {dynamicSvg.SvgIcon && (
        <span>
          <dynamicSvg.SvgIcon {...{
            ...svgProp,
            fill: color,
          }} />
        </span>
      )}
    </>
  );
}