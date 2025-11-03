import { useTranslateModals } from "@hooks/useTranslate.hook";
import { Box, Group, Button, Progress, Text, Stack } from "@mantine/core";
import { TauriTypes } from "$types";
import { RichTextEditor } from "@mantine/tiptap";
import { useEditor } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import { Markdown } from "tiptap-markdown";
import { useState } from "react";
import { Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
export interface UpdateAvailableModalProps {
  app_info: TauriTypes.AppInfo | undefined;
  context?: string;
  is_manual?: boolean;
  new_version?: string;
  download_url?: string;
  updater?: Update;
}

export function UpdateAvailableModal({ is_manual, download_url, context, updater, new_version }: UpdateAvailableModalProps) {
  const useTranslateModal = (key: string, ctx?: Record<string, any>, raw?: boolean) => useTranslateModals(`update_available.${key}`, ctx, raw);
  const [downloadProgress, setDownloadProgress] = useState(0);
  const [isDownloading, setIsDownloading] = useState(false);
  const [downloaded, setDownloaded] = useState(0);
  const [contentLength, setContentLength] = useState(0);

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  };

  const editor = useEditor({
    extensions: [StarterKit, Markdown],
    editable: false,
    content: context,
  });

  return (
    <Box>
      <RichTextEditor editor={editor} maw="100%" mah="60vh" style={{ border: "none" }}>
        <RichTextEditor.Content
          mah="55vh"
          style={{
            overflow: "auto",
            maxWidth: "100%",
          }}
        />
      </RichTextEditor>

      {isDownloading && (
        <Stack gap="xs" mt="md">
          <Progress value={downloadProgress} size="lg" radius="md" striped animated={downloadProgress < 100} />
          {(downloaded > 0 || contentLength > 0) && (
            <Group justify="space-between">
              <Text size="xs" c="dimmed">
                {formatBytes(downloaded)} / {formatBytes(contentLength)}
              </Text>
              <Text size="xs" c="dimmed">
                {contentLength > 0 ? `${((downloaded / contentLength) * 100).toFixed(1)}%` : "0%"}
              </Text>
            </Group>
          )}
        </Stack>
      )}

      <Group grow justify="space-between" mt={"md"}>
        <Button
          loading={isDownloading}
          onClick={async () => {
            if (is_manual) return open(download_url);
            if (!updater) return;
            if (isDownloading) return;
            setIsDownloading(true);
            setDownloadProgress(0);

            let downloaded = 0;
            let contentLength: number | undefined = 0;
            await updater.downloadAndInstall((event) => {
              switch (event.event) {
                case "Started":
                  setContentLength(event.data.contentLength || 0);
                  contentLength = event.data.contentLength;
                  break;
                case "Progress":
                  downloaded += event.data.chunkLength;
                  const progress = contentLength ? Math.round((downloaded / contentLength) * 100) : 0;
                  setDownloadProgress(progress);
                  setDownloaded(downloaded);
                  break;
                case "Finished":
                  setDownloadProgress(100);
                  setIsDownloading(false);
                  break;
              }
            });
            await relaunch();
          }}
        >
          {is_manual ? useTranslateModal("buttons.download_manual") : useTranslateModal("buttons.download")}
        </Button>
        <Button
          disabled={isDownloading}
          onClick={async () => {
            open(`https://github.com/Kenya-DK/quantframe-react/releases/tag/v${updater?.version || new_version}`);
          }}
        >
          {useTranslateModal("buttons.read_more")}
        </Button>
      </Group>
    </Box>
  );
}
