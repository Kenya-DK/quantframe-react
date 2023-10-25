
import { Box, createStyles, Text } from '@mantine/core';
import { Wfm } from '../types';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faArrowsRotate } from '@fortawesome/free-solid-svg-icons';
import SvgIcon, { SvgType } from './SvgIcon';
interface RivenPreviewProps {
  riven: Wfm.RivenItemDto;
}
const useStyles = createStyles(() => ({
  polarity: {
    position: "absolute", right: "15%", top: "9.7%"
  },
  itemName: {
    position: "absolute", top: "50%", fontSize: 16, fontWeight: 700
  },
  modName: {
    position: "absolute", top: "calc(50% + 1.2rem)", fontSize: 16, fontWeight: 700
  },
  attributes: {
    position: "absolute", top: "calc(50% + 3rem)", fontSize: 15
  },
  attributeText: {
    lineHeight: "1.2rem"
  },
  mastery: {
    position: "absolute", bottom: "12%", left: "21%", fontWeight: 600
  },
  reRolls: {
    position: "absolute", bottom: "12%", right: "20%", fontWeight: 600
  },
  modRank: {
    position: "absolute", bottom: "2.1%", left: "34.1%"
  },
  modLevelCircle: {
    marginLeft: "4.3px",
    color: '#c6eaff',
    textShadow: "0 0 7px #fff;",
  },
}));
export const RivenPreview = ({ riven }: RivenPreviewProps) => {
  const { classes } = useStyles();
  return (
    <Box w={316} h={400} style={{
      position: 'relative',
      display: 'flex',
      alignItems: 'center',
      flexDirection: 'column',
      backgroundImage: `url(/riven_template.png)`,
      color: '#c3aae5',
    }} >
      <SvgIcon wrapperStyle={classes.polarity} svgProp={{
        fill: '#c3aae5',
        width: 16,
        height: 16,
      }} iconType={SvgType.Polaritys} iconName={riven.polarity} />
      <Text className={classes.itemName}>{riven.weapon_name}</Text>
      <Text className={classes.modName}>{riven.mod_name}</Text>
      <Box className={classes.attributes} style={{
        display: 'flex',
        alignItems: 'center',
        flexDirection: 'column',
      }}>
        {riven.attributes.map((item, index) => {
          return (
            <Text key={index} className={classes.attributeText}>
              {item.value}{item.units == "percent" ? "%" : ""} {item.effect}
            </Text>
          )
        })}
      </Box>
      <Text className={classes.mastery}>MR {riven.mastery_rank}</Text>
      {riven.re_rolls > 0 &&
        <Text className={classes.reRolls}>
          <FontAwesomeIcon icon={faArrowsRotate} />
          <Text component="span" ml={5}>
            {riven.re_rolls}
          </Text>
        </Text>
      }
      <Box className={classes.modRank}>
        {Array.from(Array(riven.mod_rank).keys()).map((i) => {
          return <Text key={i} className={classes.modLevelCircle} size="sm" component="span">●</Text>
        })}
      </Box>
    </Box>
  );
}