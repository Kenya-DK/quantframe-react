import { modals } from "@mantine/modals";
import { Text } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateCommon, useTranslateModals } from "@hooks/useTranslate.hook";
import { GenerateTradeMessageModal, GenerateTradeMessageModalProps } from "@components/Modals/GenerateTradeMessage";
import { UpdateTradeEntry } from "@components/Forms/UpdateTradeEntry";

interface ModalHooks {
  updateMutation: {
    mutateAsync: (data: TauriTypes.UpdateTradeEntry) => Promise<any>;
  };
  updateMultipleMutation: {
    mutateAsync: (data: { ids: number[]; input: TauriTypes.UpdateTradeEntry }) => Promise<any>;
  };
  deleteMutation: {
    mutateAsync: (id: number) => Promise<any>;
  };
  deleteMultipleMutation: {
    mutateAsync: (ids: number[]) => Promise<any>;
  };
}

export const useModals = ({ updateMutation, updateMultipleMutation, deleteMutation, deleteMultipleMutation }: ModalHooks) => {
  const OpenPriceModal = (id: number, price: number) => {
    modals.openContextModal({
      modalKey: "prompt",
      title: useTranslateCommon("prompts.price.title"),
      innerProps: {
        fields: [
          {
            name: "price",
            label: useTranslateCommon("prompts.price.fields.price.label"),
            attributes: {
              min: 0,
              description: useTranslateCommon("prompts.price.fields.price.description"),
            },
            value: price,
            type: "number",
          },
        ],
        onConfirm: async (data: { price: number }) => {
          if (!id) return;
          const { price } = data;
          await updateMutation.mutateAsync({ id, price });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };
  const OpenGenerateTradeMessageModal = (input: GenerateTradeMessageModalProps) => {
    modals.open({
      size: "100%",
      title: useTranslateModals("generate_trade_message.title", { count: input.items.length }),
      withCloseButton: false,
      children: <GenerateTradeMessageModal {...input} />,
    });
  };
  const OpenUpdateMultipleModal = (ids: number[]) => {
    let id = modals.open({
      size: "100%",
      withCloseButton: false,
      children: (
        <UpdateTradeEntry
          values={ids}
          onUpdate={async (input) => {
            if (ids.length == 1) await updateMutation.mutateAsync(input);
            else await updateMultipleMutation.mutateAsync({ ids, input: { ...input, id: -1 } });
            modals.close(id);
          }}
        />
      ),
    });
  };

  const OpenDeleteModal = (id: number) => {
    modals.openConfirmModal({
      title: useTranslateCommon("prompts.delete_item.title"),
      children: <Text size="sm">{useTranslateCommon("prompts.delete_item.message", { count: 1 })}</Text>,
      labels: { confirm: useTranslateCommon("prompts.delete_item.confirm"), cancel: useTranslateCommon("prompts.delete_item.cancel") },
      onConfirm: async () => await deleteMutation.mutateAsync(id),
    });
  };

  const OpenDeleteMultipleModal = (ids: number[]) => {
    modals.openConfirmModal({
      title: useTranslateCommon("prompts.delete_multiple_items.title"),
      children: <Text size="sm">{useTranslateCommon("prompts.delete_multiple_items.message", { count: ids.length })}</Text>,
      labels: {
        confirm: useTranslateCommon("prompts.delete_multiple_items.confirm"),
        cancel: useTranslateCommon("prompts.delete_multiple_items.cancel"),
      },
      onConfirm: async () => await deleteMultipleMutation.mutateAsync(ids),
    });
  };

  return {
    OpenPriceModal,
    OpenUpdateMultipleModal,
    OpenGenerateTradeMessageModal,
    OpenDeleteModal,
    OpenDeleteMultipleModal,
  };
};
