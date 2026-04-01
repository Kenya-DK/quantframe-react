import { Box, BoxProps, ElementProps, factory, Factory, MantineSize, Select, StylesApiProps, useProps, useStyles } from "@mantine/core";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faArrowDownWideShort, faArrowUpWideShort } from "@fortawesome/free-solid-svg-icons";
import classes from "./CompactSortSelect.module.css";

type SortDirection = "asc" | "desc";

export type CompactSortSelectStylesNames = "root" | "leftIcon" | "select" | "selectInput" | "selectSection" | "directionIcon";

export interface CompactSortSelectProps extends BoxProps, StylesApiProps<CompactSortSelectFactory>, ElementProps<"div", "onChange"> {
  value: string;
  data: { label: string; value: string }[];
  direction: SortDirection;
  size?: MantineSize | (string & {});
  onChange: (value: string) => void;
  onDirectionChange: (direction: SortDirection) => void;
}

export type CompactSortSelectFactory = Factory<{
  props: CompactSortSelectProps;
  ref: HTMLDivElement;
  stylesNames: CompactSortSelectStylesNames;
}>;

const defaultProps: Partial<CompactSortSelectProps> = {};

export const CompactSortSelect = factory<CompactSortSelectFactory>((_props) => {
  const props = useProps("CompactSortSelect", defaultProps, _props);
  const { classNames, className, style, styles, unstyled, vars, attributes, value, data, direction, onChange, onDirectionChange, ref, ...others } =
    props;

  const getStyles = useStyles<CompactSortSelectFactory>({
    name: "CompactSortSelect",
    classes,
    props,
    className,
    style,
    classNames,
    styles,
    unstyled,
    vars,
    attributes,
  });

  const toggleDirection = () => onDirectionChange(direction === "asc" ? "desc" : "asc");

  return (
    <Box ref={ref} {...getStyles("root")} {...others}>
      <FontAwesomeIcon
        className={getStyles("directionIcon").className}
        icon={direction === "asc" ? faArrowDownWideShort : faArrowUpWideShort}
        onClick={() => toggleDirection()}
      />
      <Select
        classNames={{
          root: getStyles("select").className,
          input: getStyles("selectInput").className,
          section: getStyles("selectSection").className,
        }}
        size="xs"
        allowDeselect={false}
        value={value}
        onChange={(nextValue) => {
          if (nextValue) onChange(nextValue);
        }}
        data={data}
      />
    </Box>
  );
});

CompactSortSelect.displayName = "@quantframe/CompactSortSelect";
CompactSortSelect.classes = classes;
