import { modals } from "@mantine/modals";
import { Text } from "@mantine/core";

interface ModalHooks {
  useTranslateBasePrompt: (key: string, context?: { [key: string]: any }) => string;
  deleteMutation: {
    mutateAsync: (id: string) => Promise<any>;
  };
}

export const useModals = ({ useTranslateBasePrompt, deleteMutation }: ModalHooks) => {
  const OpenDeleteModal = (id: string) => {
    modals.openConfirmModal({
      title: useTranslateBasePrompt("delete_chat.title"),
      children: <Text size="sm">{useTranslateBasePrompt("delete_chat.message", { count: 1 })}</Text>,
      labels: { confirm: useTranslateBasePrompt("delete_chat.confirm"), cancel: useTranslateBasePrompt("delete_chat.cancel") },
      onConfirm: async () => await deleteMutation.mutateAsync(id),
    });
  };

  return {
    OpenDeleteModal,
  };
};
