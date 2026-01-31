import { modals } from "@mantine/modals";
import { Text } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateCommon } from "@hooks/useTranslate.hook";

interface ModalHooks {
  createMutation: {
    mutateAsync: (data: { settings: TauriTypes.Settings; template: TauriTypes.SaveTemplateSetting }) => Promise<any>;
  };
  deleteMutation: {
    mutateAsync: (data: { settings: TauriTypes.Settings; name: string }) => Promise<any>;
  };
}

export const useModals = ({ createMutation, deleteMutation }: ModalHooks) => {
  const OpenSaveModal = (input: { settings: TauriTypes.Settings; template: Omit<TauriTypes.SaveTemplateSetting, "name"> }) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateCommon("prompts.save.title"),
      innerProps: {
        fields: [
          {
            name: "name",
            label: useTranslateCommon("prompts.save.fields.name.label"),
            value: "",
            type: "text",
          },
        ],
        onConfirm: async (data: { name: string }) => {
          if (!input.template) return;
          const { name } = data;
          if (input.settings?.generate_trade_message.templates.find((t) => t.name === name)) return;
          await createMutation.mutateAsync({ settings: input.settings, template: { ...input.template, name } });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };

  const OpenDeleteModal = (settings: TauriTypes.Settings, name: string) => {
    modals.openConfirmModal({
      title: useTranslateCommon("prompts.delete_item.title"),
      children: <Text size="sm">{useTranslateCommon("prompts.delete_item.message", { count: 1 })}</Text>,
      labels: { confirm: useTranslateCommon("prompts.delete_item.confirm"), cancel: useTranslateCommon("prompts.delete_item.cancel") },
      onConfirm: async () => await deleteMutation.mutateAsync({ settings, name }),
    });
  };

  return {
    OpenDeleteModal,
    OpenSaveModal,
  };
};
