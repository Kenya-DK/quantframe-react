import { Group, NumberInput } from "@mantine/core";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faLeftRight, faInfinity } from "@fortawesome/free-solid-svg-icons";
import styles from "./MinMax.module.css";

export type MinMaxProps = {
  value: [number | undefined, number | null | undefined] | undefined;
  minAllowed?: number;
  maxAllowed?: number;
  label: string;
  description?: string;
  onChange: (value: [number | undefined, number | null | undefined] | undefined) => void;
};

export function MinMax({ value, label, description, minAllowed, maxAllowed, onChange }: MinMaxProps) {
  return (
    <Group gap={"sm"}>
      <NumberInput
        value={value?.[0]}
        w={"100px"}
        min={minAllowed ?? -999999999}
        max={maxAllowed ?? 999999999}
        onChange={(n) => {
          const max = value?.[1] ?? undefined;
          if (n === "") onChange([undefined, max]);
          const min = Number(n);
          onChange([min, max]);
        }}
        label={label}
        description={description}
        placeholder="0"
      />
      <FontAwesomeIcon icon={faLeftRight} style={{ marginTop: 23 }} />
      <NumberInput
        mt={23}
        w={"100px"}
        value={value?.[1] ?? ""}
        min={minAllowed ?? 0}
        max={maxAllowed ?? 999999999}
        onChange={(n) => {
          const min = value?.[0] ?? undefined;
          if (n === "") onChange([min, undefined]);
          const max = Number(n);
          onChange([min, max]);
        }}
        placeholder="No limit"
        rightSection={!value?.[1] && value?.[1] !== 0 ? <FontAwesomeIcon icon={faInfinity} className={styles.infinityIcon} /> : undefined}
      />
    </Group>
  );
}
