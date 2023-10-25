import { useEffect, useState } from "react";
import { useDynamicSvgImport, useDynamicPolaritySvgImport } from "@hooks/index";

// SvgType is an enum that is used to determine which svg icon to render
export enum SvgType {
  Default = "default",
  Faction = "factions",
  Polaritys = "polaritys",
}

interface IProps {
  iconName: string;
  iconType: SvgType;
  wrapperStyle?: string;
  svgProp?: React.SVGProps<SVGSVGElement>;
  seleteded?: boolean;
}

function SvgIcon(props: IProps) {
  const { iconType } = props;
  return (
    <>
      {iconType == SvgType.Default && (
        <SvgIconDefault {...props} />
      )}
      {iconType == SvgType.Polaritys && (
        <SvgIconPolaritys {...props} />
      )}
    </>
  )
}
export default SvgIcon;


// SvgIconDefault is a component that is used to render the svg icon
function SvgIconDefault(props: IProps) {
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
function SvgIconPolaritys(props: IProps) {
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
interface ICoreProps {
  dynamicSvg: {
    error: any, loading: boolean, SvgIcon: React.FC<React.SVGProps<SVGElement>> | undefined
  }
  wrapperStyle?: string;
  svgProp?: React.SVGProps<SVGSVGElement>;
  seleteded?: boolean;
}
function SvgCore(props: ICoreProps) {
  const { wrapperStyle, svgProp, seleteded, dynamicSvg } = props;
  const [color, setColor] = useState<string>("gray");

  useEffect(() => {
    if (svgProp?.fill)
      setColor(svgProp.fill);
    else if (seleteded)
      setColor("#61dafb");
    else
      setColor("gray");
  }, [seleteded, svgProp]);

  return (
    <>
      {dynamicSvg.error && (
        <div className="rounded-full bg-slate-400 h-8 w-8">{JSON.stringify(dynamicSvg.error)}</div>
      )}
      {dynamicSvg.loading && (
        <div className="rounded-full bg-slate-400 animate-pulse h-8 w-8"></div>
      )}
      {dynamicSvg.SvgIcon && (
        <div className={wrapperStyle}>
          <dynamicSvg.SvgIcon {...{
            ...svgProp,
            fill: color,
          }} />
        </div>
      )}
    </>
  );
}
