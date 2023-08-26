import { Settings } from "$types/index";
interface GeneralProps {
  settings: Settings | undefined;
  updateSettings: (user: Partial<Settings>) => void;
}

export function GeneralPanel({ }: GeneralProps) {


  return (
    <>
      KKK
    </>
  );
}