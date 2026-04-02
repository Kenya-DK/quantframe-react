import { modals } from "@mantine/modals";
import { Text } from "@mantine/core";
import { useTranslateCommon } from "@hooks/useTranslate.hook";
import { WFMarketTypes } from "$types";
import { Operations, RivenDetailsModal } from "@components/Modals/RivenDetails";

interface ModalHooks {
  useTranslateBasePrompt: (key: string, context?: { [key: string]: any }) => string;
  importStockMutation: {
    mutateAsync: (data: { id: string; bought: number }) => Promise<any>;
  };
  deleteStockMutation: {
    mutateAsync: (id: string) => Promise<any>;
  };
}

export const useStockModals = ({ deleteStockMutation, importStockMutation }: ModalHooks) => {
  const OpenImportModal = (id: string) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateCommon("prompts.bought_manual.title"),
      innerProps: {
        fields: [
          {
            name: "bought",
            label: useTranslateCommon("prompts.bought_manual.fields.bought.label"),
            attributes: {
              min: 0,
            },
            value: 0,
            type: "number",
          },
        ],
        onConfirm: async (data: { bought: number }) => {
          if (!id) return;
          const { bought } = data;
          await importStockMutation.mutateAsync({ id, bought });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };
  const OpenDeleteModal = (id: string) => {
    modals.openConfirmModal({
      title: useTranslateCommon("prompts.delete_item.title"),
      children: <Text size="sm">{useTranslateCommon("prompts.delete_item.message", { count: 1 })}</Text>,
      labels: { confirm: useTranslateCommon("prompts.delete_item.confirm"), cancel: useTranslateCommon("prompts.delete_item.cancel") },
      onConfirm: async () => await deleteStockMutation.mutateAsync(id),
    });
  };
  const OpenInfoModal = (item: WFMarketTypes.Auction) => {
    modals.open({
      size: "100%",
      withCloseButton: false,
      children: <RivenDetailsModal value={item.id} lookup="auction" operations={[Operations.MarketInfo, Operations.TransactionInfo]} />,
    });
  };

  return {
    OpenInfoModal,
    OpenImportModal,
    OpenDeleteModal,
  };
};
