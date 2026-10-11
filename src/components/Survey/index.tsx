import { TauriTypes } from "$types";
import api from "@api/index";
import { useIsAuthenticated } from "@hooks/useIsAuthenticated.hook";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
import { Button, Checkbox, Group, Radio, Stack, Text, Textarea } from "@mantine/core";
import { modals } from "@mantine/modals";
import { notifications } from "@mantine/notifications";
import { useEffect, useState } from "react";

export type SurveyModalProps = {
  survey: TauriTypes.Survey;
  onSubmitted: () => void;
};

export function SurveyModal({ survey, onSubmitted }: SurveyModalProps) {
  const useTranslate = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`modals.survey.${key}`, { ...context });
  const [answers, setAnswers] = useState<Record<string, string | string[]>>({});
  const [submitting, setSubmitting] = useState(false);

  const setAnswer = (id: string, value: string | string[]) => setAnswers((prev) => ({ ...prev, [id]: value }));

  const isAnswered = (question: TauriTypes.SurveyQuestion) => {
    const value = answers[question.id];
    if (Array.isArray(value)) return value.length > 0;
    return typeof value === "string" && value.trim().length > 0;
  };

  const canSubmit = survey.questions.every((question) => !question.required || isAnswered(question));

  const handleSubmit = async () => {
    setSubmitting(true);
    try {
      await api.survey.submit(survey.id, answers);
      notifications.show({ title: useTranslate("success.title"), message: useTranslate("success.message"), color: "green.7" });
      onSubmitted();
    } catch (e) {
      console.error(e);
      notifications.show({ title: useTranslate("error.title"), message: useTranslate("error.message"), color: "red.7" });
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <Stack>
      {survey.description && <Text c="dimmed">{survey.description}</Text>}
      {survey.questions.map((question) => (
        <Stack key={question.id} gap={4}>
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
          {question.type === "single" && (
            <Radio.Group value={(answers[question.id] as string) || ""} onChange={(value) => setAnswer(question.id, value)}>
              <Stack gap={4}>
                {question.options?.map((option) => (
                  <Radio key={option.id} value={option.id} label={option.label} />
                ))}
              </Stack>
            </Radio.Group>
          )}
          {question.type === "multiple" && (
            <Checkbox.Group value={(answers[question.id] as string[]) || []} onChange={(value) => setAnswer(question.id, value)}>
              <Stack gap={4}>
                {question.options?.map((option) => (
                  <Checkbox key={option.id} value={option.id} label={option.label} />
                ))}
              </Stack>
            </Checkbox.Group>
          )}
          {question.type === "text" && (
            <Textarea
              value={(answers[question.id] as string) || ""}
              onChange={(event) => setAnswer(question.id, event.currentTarget.value)}
              placeholder={question.placeholder}
              maxLength={question.max_length}
              autosize
              minRows={2}
            />
          )}
        </Stack>
      ))}
      <Group justify="flex-end">
        <Button onClick={handleSubmit} loading={submitting} disabled={!canSubmit}>
          {useTranslate("submit")}
        </Button>
      </Group>
    </Stack>
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

        let modalId = "";
        modalId = modals.open({
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
