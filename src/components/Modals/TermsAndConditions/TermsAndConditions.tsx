import { RichTextEditor } from "@mantine/tiptap";
import { useEditor } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import { Markdown } from "tiptap-markdown";
import { useTranslateModals } from "@hooks/useTranslate.hook";
import { Button, Container, Group } from "@mantine/core";
import classes from "./TermsAndConditions.module.css";
export type TermsAndConditionsProps = {
  content?: string;
  onAccept?: () => void;
  onDecline?: () => void;
};
export function TermsAndConditions({ content, onAccept, onDecline }: TermsAndConditionsProps) {
  // Translate general
  const useTranslateTOS = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`tos.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTOS(`buttons.${key}`, { ...context }, i18Key);
  const editor = useEditor({
    extensions: [StarterKit, Markdown],
    editable: false,
    content: content,
  });
  return (
    <Container p={0} m={0} size={"100%"}>
      <RichTextEditor editor={editor} variant="subtle" className={classes.editor}>
        <RichTextEditor.Content />
      </RichTextEditor>
      <Group justify="flex-end" mt={"md"}>
        <Button
          onClick={() => {
            onDecline && onDecline();
          }}
        >
          {useTranslateButtons("decline")}
        </Button>
        <Button
          onClick={() => {
            onAccept && onAccept();
          }}
        >
          {useTranslateButtons("accept")}
        </Button>
      </Group>
    </Container>
  );
}
