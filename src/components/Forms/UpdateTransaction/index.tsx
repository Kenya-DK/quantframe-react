import { Box, Button, Collapse, JsonInput } from "@mantine/core";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { TauriTypes } from "$types";
import { DateTimePicker } from "@mantine/dates";
import dayjs from "dayjs";
import utc from "dayjs/plugin/utc";
import { useState } from "react";
import { DynamicForm } from "../DynamicForm";

dayjs.extend(utc);

export type UpdateTransactionProps = {
  value?: TauriTypes.TransactionDto;
  onSubmit: (values: TauriTypes.UpdateTransaction) => void;
};

function PropertiesField({ value, onChange }: { value: any; onChange: (value: any) => void }) {
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`update_transaction.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);

  const [text, setText] = useState<string>(value ? JSON.stringify(value, null, 2) : "");
  const [show, setShow] = useState<boolean>(false);

  return (
    <>
      <Button mt="md" onClick={() => setShow((prev) => !prev)}>
        {show ? useTranslateForm("buttons.hide_properties.label") : useTranslateForm("buttons.show_properties.label")}
      </Button>
      <Collapse expanded={show}>
        <JsonInput
          label={useTranslateFields("properties.label")}
          description={useTranslateFields("properties.description")}
          placeholder={useTranslateFields("properties.placeholder")}
          value={text}
          onChange={(event) => {
            setText(event);
            try {
              onChange(JSON.parse(event));
            } catch {
              // ignore invalid JSON until it parses
            }
          }}
          radius="md"
          mt="md"
          autosize
          minRows={4}
        />
      </Collapse>
    </>
  );
}

export function UpdateTransaction({ value, onSubmit }: UpdateTransactionProps) {
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`update_transaction.fields.${key}`, { ...context }, i18Key);

  return (
    <Box w={"100%"}>
      <DynamicForm<TauriTypes.TransactionDto>
        value={value}
        i18n_key="components.forms.update_transaction.fields"
        items={[
          { field: "price", type: "number", props: { min: 0 } },
          { field: "quantity", type: "number", props: { min: 0 } },
          { field: "user_name", type: "text", props: { mt: "md" } },
          {
            field: "created_at",
            type: "custom",
            render: ({ value, onChange }) => (
              <DateTimePicker
                label={useTranslateFormFields("created_at.label")}
                description={useTranslateFormFields("created_at.description")}
                placeholder={useTranslateFormFields("created_at.placeholder")}
                value={value ? new Date(value) : null}
                onChange={(event) => onChange(event ? dayjs(event).utc().toISOString() : undefined)}
                radius="md"
                mt="md"
              />
            ),
          },
          {
            field: "properties",
            type: "custom",
            render: ({ value, onChange }) => <PropertiesField value={value} onChange={onChange} />,
          },
        ]}
        onSubmit={(values) => onSubmit(values as TauriTypes.UpdateTransaction)}
        confirmLabel={useTranslateCommon("buttons.save.label")}
      />
    </Box>
  );
}
