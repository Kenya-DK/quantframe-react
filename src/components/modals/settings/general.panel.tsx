import { Image } from "@mantine/core";
import { Settings } from "$types/index";
interface GeneralProps {
  settings: Settings | undefined;
  updateSettings: (user: Partial<Settings>) => void;
}

export function GeneralPanel({ }: GeneralProps) {


  return (
    <>
      <Image src={"https://cataas.com/cat"} radius="md"
        h={30}
        w="auto"
        fit="contain" />
    </>
  );
}