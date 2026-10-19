import { TauriTypes } from "$types";
import api from "@api/index";
import { useIsAuthenticated } from "@hooks/useIsAuthenticated.hook";
import { useTranslateModals } from "@hooks/useTranslate.hook";
import { Button, Card, Checkbox, Divider, Group, Radio, ScrollArea, Stack, Text, Textarea } from "@mantine/core";
import { FormErrors, useForm, UseFormReturnType } from "@mantine/form";
import { modals } from "@mantine/modals";
import { notifications } from "@mantine/notifications";
import { useEffect, useState } from "react";

export type SurveyModalProps = {
  survey: TauriTypes.Survey;
  onSubmitted: () => void;
};

type AnswerValue = string | string[];

type SurveyFormValues = { answers: AnswerValue[] };
type SurveyTransformedValues = { answers: Record<string, AnswerValue> };

const emptyAnswer = (question: TauriTypes.SurveyQuestion): AnswerValue => (question.type === "multiple" ? [] : "");

const isEmptyAnswer = (value: AnswerValue | undefined) => {
  if (Array.isArray(value)) return value.length === 0;
  return typeof value !== "string" || value.trim().length === 0;
};

type SurveyQuestionFieldProps = {
  question: TauriTypes.SurveyQuestion;
  index: number;
  form: UseFormReturnType<SurveyFormValues, SurveyTransformedValues>;
};

/** Renders a single survey question bound to its field in the survey form. */
function SurveyQuestionField({ question, index, form }: SurveyQuestionFieldProps) {
  const path = `answers.${index}`;
  const inputProps = form.getInputProps(path);
  const value = form.useWatchValue(path);
  const textValue = typeof value === "string" ? value : "";

  return (
    <Card withBorder radius="md" padding="md">
      <Stack gap="xs">
        <Stack gap={2}>
          <Text fw={600}>
            {question.title}
            {question.required && (
              <Text span c="red.7">
                {" *"}
              </Text>
            )}
          </Text>
          {question.description && (
            <Text size="sm" c="dimmed">
              {question.description}
            </Text>
          )}
        </Stack>

        {question.type === "single" && (
          <Radio.Group key={form.key(path)} {...inputProps}>
            <Stack gap="xs" pl="xs">
              {question.options?.map((option) => (
                <Radio key={option.id} value={option.id} label={option.label} />
              ))}
            </Stack>
          </Radio.Group>
        )}

        {question.type === "multiple" && (
          <Checkbox.Group key={form.key(path)} {...inputProps}>
            <Stack gap="xs" pl="xs">
              {question.options?.map((option) => (
                <Checkbox key={option.id} value={option.id} label={option.label} />
              ))}
            </Stack>
          </Checkbox.Group>
        )}

        {question.type === "text" && (
          <Stack gap={4}>
            <Textarea
              key={form.key(path)}
              {...inputProps}
              placeholder={question.placeholder}
              maxLength={question.max_length}
              autosize
              minRows={2}
              maxRows={6}
            />
            {question.max_length != null && (
              <Text size="xs" c="dimmed" ta="right">
                {textValue.length}/{question.max_length}
              </Text>
            )}
          </Stack>
        )}
      </Stack>
    </Card>
  );
}

export function SurveyModal({ survey, onSubmitted }: SurveyModalProps) {
  const useTranslate = (key: string, context?: { [key: string]: any }) => useTranslateModals(`survey.${key}`, { ...context });

  const form = useForm<SurveyFormValues, SurveyTransformedValues>({
    mode: "uncontrolled",
    initialValues: { answers: survey.questions.map(emptyAnswer) },
    validate: (values) => {
      const errors: FormErrors = {};
      survey.questions.forEach((question, index) => {
        if (question.required && isEmptyAnswer(values.answers[index])) {
          errors[`answers.${index}`] = useTranslate("required");
        }
      });
      return errors;
    },
    transformValues: (values) => ({
      answers: survey.questions.reduce<Record<string, AnswerValue>>((acc, question, index) => {
        acc[question.id] = values.answers[index];
        return acc;
      }, {}),
    }),
  });

  const handleSubmit = async (values: SurveyTransformedValues) => {
    try {
      await api.survey.submit(survey.id, values.answers);
      notifications.show({ title: useTranslate("success.title"), message: useTranslate("success.message"), color: "green.7" });
      onSubmitted();
    } catch (e) {
      console.error(e);
      notifications.show({ title: useTranslate("error.title"), message: useTranslate("error.message"), color: "red.7" });
    }
  };

  return (
    <form onSubmit={form.onSubmit(handleSubmit)}>
      <Stack gap="md">
        {survey.description && (
          <Text size="sm" c="dimmed">
            {survey.description}
          </Text>
        )}

        <ScrollArea.Autosize mah="60vh" type="hover" offsetScrollbars>
          <Stack gap="md" pr="sm">
            {survey.questions.map((question, index) => (
              <SurveyQuestionField key={question.id} question={question} index={index} form={form} />
            ))}
          </Stack>
        </ScrollArea.Autosize>

        <Divider />
        <Group justify="flex-end">
          <Button type="submit" loading={form.submitting}>
            {useTranslate("submit")}
          </Button>
        </Group>
      </Stack>
    </form>
  );
}

/**
 * Fetches the active survey once the user is logged in and opens it as a modal.
 * Renders nothing.
 */
export function SurveyGate() {
  const isAuthenticated = useIsAuthenticated();
  const [shownSurvey, setShownSurvey] = useState<string | null>(null);

  useEffect(() => {
    if (!isAuthenticated) return;
    let cancelled = false;

    (async () => {
      try {
        const survey = await api.survey.getActive();
        if (cancelled || !survey || survey.id === shownSurvey) return;
        setShownSurvey(survey.id);

        const modalId = modals.open({
          title: survey.title,
          centered: true,
          size: "lg",
          withCloseButton: survey.dismissible,
          closeOnClickOutside: survey.dismissible,
          closeOnEscape: survey.dismissible,
          children: <SurveyModal survey={survey} onSubmitted={() => modals.close(modalId)} />,
        });
      } catch (e) {
        console.error("Failed to load active survey:", e);
      }
    })();

    return () => {
      cancelled = true;
    };
  }, [isAuthenticated]);

  return null;
}
