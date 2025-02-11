import { Center, LoadingOverlay } from "@mantine/core";
import classes from "./PremiumOverlay.module.css";
export type PremiumOverlayProps = {
  text?: string;
};
export function PremiumOverlay({}: PremiumOverlayProps) {
  return (
    <LoadingOverlay
      visible
      overlayProps={{ radius: "sm", blur: 2 }}
      loaderProps={{
        children: (
          <Center classNames={classes}>
            <div className={classes.loader}>
              <div className={classes.spin}></div>
              <div className={classes.bounce}></div>
            </div>
          </Center>
        ),
      }}
    ></LoadingOverlay>
  );
}
