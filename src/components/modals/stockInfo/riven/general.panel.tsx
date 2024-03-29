import { Image } from "@mantine/core";
interface GeneralProps { }

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