import { Center } from '@mantine/core';
import classes from './Loading.module.css';
export type LoadingProps = {
  text?: string;
}
export function Loading({ }: LoadingProps) {

  return (
    <Center classNames={classes}>
      <div className={classes.loader}>
        <div className={classes.spin}></div>
        <div className={classes.bounce}></div>
      </div>
    </Center>
  );
}