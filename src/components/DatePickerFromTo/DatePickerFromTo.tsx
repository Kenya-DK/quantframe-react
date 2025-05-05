import { DatePickerInput } from "@mantine/dates";
export type DatePickerFromToProps = {
  label: string;
  description?: string;
  from_date: string | Date | undefined;
  to_date: string | Date | undefined;
  onDatesChange: ({ from_date, to_date }: { from_date: Date | null; to_date: Date | null }) => void;
};
export function DatePickerFromTo({ description, label, from_date, to_date, onDatesChange }: DatePickerFromToProps) {
  const GetDate = (): [Date | null, Date | null] => {
    if (from_date && to_date) return [new Date(from_date), new Date(to_date)];
    else if (from_date) return [new Date(from_date), null];
    else if (to_date) return [null, new Date(to_date)];
    return [null, null];
  };
  const SetValue = (dates: [Date | null, Date | null]) => {
    if (dates[0] && dates[1] && dates[0] > dates[1]) return;
    onDatesChange({ from_date: dates[0], to_date: dates[1] });
  };
  return <DatePickerInput type="range" label={label} description={description} value={GetDate()} onChange={SetValue} clearable />;
}
