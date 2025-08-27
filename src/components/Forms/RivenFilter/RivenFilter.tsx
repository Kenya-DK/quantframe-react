import { Group, Box, Button, Collapse, Switch, Grid, NumberInput, Stack } from "@mantine/core";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { TauriTypes } from "$types";
import { MinMax } from "@components/Forms/MinMax";
import { RivenFilterAttribute } from "@components/Forms/RivenFilterAttribute";

export type RivenFilterProps = {
  value: TauriTypes.StockRivenFilter;
  onSubmit: (values: TauriTypes.StockRivenFilter) => void;
};

export function RivenFilter({ value, onSubmit }: RivenFilterProps) {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`riven_filter.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);

  // User form
  const form = useForm({
    initialValues: {
      ...value,
      attributes: value.attributes ?? [],
    },
    validate: {},
  });
  return (
    <Box w={"100%"} p={"sm"}>
      <form
        onSubmit={form.onSubmit((data) => {
          onSubmit(data);
        })}
      >
        <Group gap="md">
          <Switch
            label={useTranslateFormFields("enabled.label")}
            checked={form.values.enabled}
            onChange={(event) => form.setFieldValue("enabled", event.currentTarget.checked)}
          />
        </Group>
        <Collapse in={form.values.enabled}>
          <Grid>
            <Grid.Col span={6}>
              <MinMax
                label={useTranslateFormFields("re_rolls.label")}
                value={form.values.re_rolls}
                onChange={(re_rolls) => {
                  form.setFieldValue("re_rolls", re_rolls);
                }}
              />
              <MinMax
                label={useTranslateFormFields("rank.label")}
                value={form.values.rank}
                onChange={(rank) => {
                  form.setFieldValue("rank", rank);
                }}
              />
              <MinMax
                label={useTranslateFormFields("mastery_rank.label")}
                value={form.values.mastery_rank}
                onChange={(mastery_rank) => {
                  form.setFieldValue("mastery_rank", mastery_rank);
                }}
              />
            </Grid.Col>
            <Grid.Col span={6}>
              <Group gap="md">
                <Switch
                  label={useTranslateFormFields("required_negative.label")}
                  checked={form.values.required_negative}
                  onChange={(event) => form.setFieldValue("required_negative", event.currentTarget.checked)}
                />
              </Group>
              <NumberInput
                label={useTranslateFormFields("similarity.label")}
                placeholder="0"
                value={form.values.similarity || 0}
                min={0}
                max={100}
                onChange={(event) => {
                  const value = Number(event);
                  if (value == 0) form.setFieldValue("similarity", undefined);
                  else form.setFieldValue("similarity", value);
                }}
                suffix=" %"
                mt="md"
              />
              <Stack mt="md" gap="md">
                {(form.values.attributes ?? []).map((attribute, index) => (
                  <RivenFilterAttribute
                    key={index}
                    value={attribute}
                    onChange={(value) => {
                      const attributes = [...(form.values.attributes ?? [])];
                      attributes[index] = value;
                      form.setFieldValue("attributes", attributes);
                    }}
                  />
                ))}
              </Stack>
            </Grid.Col>
          </Grid>
        </Collapse>
        {form.isDirty() && (
          <Button mt={15} type="submit" color="blue" radius="md" fullWidth>
            {useTranslateButtons("save.label")}
          </Button>
        )}
      </form>
    </Box>
  );
}
