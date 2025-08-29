import { Center, LoadingOverlay } from "@mantine/core";
import classes from "./Loading.module.css";
export type LoadingProps = {
  text?: string;
  noAnimationText?: boolean;
};
export function Loading({ text, noAnimationText }: LoadingProps) {
  const charters = [];
  for (var i = 0; i < (text || "").length; i++) charters.push(text?.[i]);
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
              <div className={classes.loaderText}>
                <div className={classes.loaderTextInner}>
                  {noAnimationText ? (
                    <span> {text}</span>
                  ) : (
                    charters.map((charter, index) => (
                      <span className={!noAnimationText ? classes.waveAnimation : ""} key={index} style={{ "--i": index + 1 } as React.CSSProperties}>
                        {" "}
                        {charter}
                      </span>
                    ))
                  )}
                </div>
              </div>
            </div>
          </Center>
        ),
      }}
    ></LoadingOverlay>
  );
}
