import { Button, ButtonProps } from "@mantine/core";
import { useLayoutEffect, useRef, useState } from "react";
import classes from "./ExpandableButton.module.css";

export type ExpandableButtonProps = Omit<ButtonProps, "children" | "rightSection"> & {
  icon: React.ReactNode;
  iconWidth?: number;
  children: React.ReactNode;
  childrenOffset?: { left?: number; right?: number };
  collapsedWidth?: number;
  expandedWidth?: number;
  transitionMs?: number;
  section?: "left" | "right";
  selected?: boolean;
  onClick?: (e: React.MouseEvent<HTMLButtonElement>) => void;
};

export function ExpandableButton({
  icon,
  children,
  collapsedWidth = 44,
  expandedWidth,
  transitionMs = 220,
  className,
  style,
  iconWidth,
  section = "left",
  selected = false,
  childrenOffset,
  onClick,
  ...buttonProps
}: ExpandableButtonProps) {
  const contentRef = useRef<HTMLSpanElement>(null);
  const [autoExpandedWidth, setAutoExpandedWidth] = useState(collapsedWidth + 176);

  useLayoutEffect(() => {
    if (typeof expandedWidth === "number") {
      setAutoExpandedWidth(expandedWidth);
      return;
    }

    const contentWidth = contentRef.current?.scrollWidth || 0;
    setAutoExpandedWidth(collapsedWidth + contentWidth + (iconWidth || 16));
  }, [children, collapsedWidth, expandedWidth, iconWidth]);

  return (
    <Button
      bg="dark.7"
      {...buttonProps}
      onClick={async (e) => onClick?.(e)}
      className={`${classes.root}${className ? ` ${className}` : ""}`}
      data-selected={selected ? "true" : undefined}
      leftSection={section === "left" ? icon : undefined}
      rightSection={section === "right" ? icon : undefined}
      style={
        {
          "--erb-collapsed-width": `${collapsedWidth}px`,
          "--erb-expanded-width": `${autoExpandedWidth}px`,
          "--erb-transition-ms": `${transitionMs}ms`,
          ...(style as React.CSSProperties),
        } as React.CSSProperties
      }
    >
      <span ref={contentRef} className={classes.content} style={{ marginLeft: childrenOffset?.left, marginRight: childrenOffset?.right }}>
        {children}
      </span>
    </Button>
  );
}
