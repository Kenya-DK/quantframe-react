import { Box, Stack, createStyles, Text } from "@mantine/core";
import { Wfm } from "$types/index";
import { useCacheContext } from "../../contexts";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCheck } from "@fortawesome/free-solid-svg-icons";
interface RivenAttributesProps {
  isClickable?: boolean,
  onClick?: (attribute: Wfm.RivenAttributeDto) => void,
  attributes: Wfm.RivenAttributeDto[]
}
const useStyles = createStyles((theme) => ({
  positiveAttributes: {
    // Child of type 
    borderColor: "rgb(#19a187/50%)",
    backgroundColor: "rgb(#19a187/10%)",
    color: "#19a187",
  },
  negativeAttributes: {
    borderColor: "rgb(#98392d/50%)",
    backgroundColor: "rgb(#98392d/10%)",
    color: "#98392d"
  },
  rivenAttributes: {
    border: "none",
    borderWidth: "1px",
    borderStyle: "solid",
    padding: "2px 10px 2px 10px",
    marginRight: theme.spacing.xs,
    borderRadius: "3px",
  },
}));
const AttributeText = ({ onClick, isClickable, attribute }: { onClick?: (attribute: Wfm.RivenAttributeDto) => void, isClickable?: boolean, attribute: Wfm.RivenAttributeDto }) => {
  const { classes, cx } = useStyles();
  const { riven_attributes } = useCacheContext();
  const getAttributeType = (url_name: string) => {
    return riven_attributes.find(x => x.url_name === url_name);
  }
  return (
    <>
      <Text style={{ cursor: isClickable ? "pointer" : "default" }} onClick={(e) => {
        e.preventDefault();
        if (!isClickable) return;
        if (onClick) onClick(attribute);
      }} span className={cx(classes.rivenAttributes, attribute.positive ? classes.positiveAttributes : classes.negativeAttributes)}>
        {attribute.match && <FontAwesomeIcon icon={faCheck} color={isClickable ? "inherit" : "gray"} />}
        {attribute.value}{getAttributeType(attribute.url_name)?.units == "percent" ? "%" : ""} {getAttributeType(attribute.url_name)?.effect}
      </Text>
    </>
  )
}
export const RivenAttributes = ({ onClick, isClickable, attributes }: RivenAttributesProps) => {
  return (
    <Stack >
      <Box>
        {attributes.filter(x => x.positive).map((att, i) => {
          return (
            <AttributeText onClick={onClick} isClickable={isClickable} key={i} attribute={att} />
          )
        })}
      </Box>
      <Box>
        {attributes.filter(x => !x.positive).map((att, i) => {
          return (
            <AttributeText onClick={onClick} isClickable={isClickable} key={i} attribute={att} />
          )
        })}
      </Box>
    </Stack>)
}