import { modals } from "@mantine/modals";
import { Text } from "@mantine/core";
import { TauriTypes } from "$types";
import { CreateRiven } from "@components/Forms/CreateRiven";
import { RivenFilter } from "@components/Forms/RivenFilter";
import { StockRivenDetailsModal } from "@components/Modals/StockRivenDetails";
import { useTranslateCommon, useTranslateModals } from "@hooks/useTranslate.hook";
import { StockRivenUpdate } from "@components/Forms/StockRivenUpdate";
import { GenerateTradeMessageModal, GenerateTradeMessageModalProps } from "@components/Modals/GenerateTradeMessage";
interface ModalHooks {
  useTranslateBasePrompt: (key: string, context?: { [key: string]: any }) => string;
  updateMutation: {
    mutateAsync: (data: TauriTypes.UpdateStockRiven) => Promise<any>;
  };
  updateMultipleMutation: {
    mutateAsync: (data: { ids: number[]; input: TauriTypes.UpdateStockRiven }) => Promise<any>;
  };
  sellMutation: {
    mutateAsync: (data: TauriTypes.SellStockRiven) => Promise<any>;
  };
  deleteMutation: {
    mutateAsync: (id: number) => Promise<any>;
  };
  deleteMultipleMutation: {
    mutateAsync: (ids: number[]) => Promise<any>;
  };
  createMutation: {
    mutateAsync: (data: TauriTypes.CreateStockRiven) => Promise<any>;
  };
}

export const useStockModals = ({
  useTranslateBasePrompt,
  updateMutation,
  updateMultipleMutation,
  sellMutation,
  deleteMutation,
  deleteMultipleMutation,
  createMutation,
}: ModalHooks) => {
  const OpenMinimumPriceModal = (id: number, minimum_price: number) => {
    modals.openContextModal({
      modalKey: "prompt",
      title: useTranslateCommon("prompts.minimum_price.title"),
      innerProps: {
        fields: [
          {
            name: "minimum_price",
            label: useTranslateCommon("prompts.minimum_price.fields.minimum_price.label"),
            attributes: {
              min: 0,
              description: useTranslateCommon("prompts.minimum_price.fields.minimum_price.description"),
            },
            value: minimum_price,
            type: "number",
          },
        ],
        onConfirm: async (data: { minimum_price: number }) => {
          if (!id) return;
          const { minimum_price } = data;
          await updateMutation.mutateAsync({ id, minimum_price });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };

  const OpenSellModal = (stock: TauriTypes.SellStockRiven) => {
    modals.openContextModal({
      modalKey: "prompt",
      title: useTranslateCommon("prompts.sell_manual.title"),
      innerProps: {
        fields: [
          {
            name: "sell",
            label: useTranslateCommon("prompts.sell_manual.fields.sell.label"),
            attributes: {
              min: 0,
            },
            value: 0,
            type: "number",
          },
        ],
        onConfirm: async (data: { sell: number }) => {
          if (!stock) return;
          const { sell } = data;
          await sellMutation.mutateAsync({ ...stock, price: sell });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };

  const OpenInfoModal = (item: TauriTypes.StockRiven) => {
    modals.open({
      size: "100%",
      withCloseButton: false,
      children: <StockRivenDetailsModal value={item.id} />,
    });
  };
  const OpenWTSModal = (input: GenerateTradeMessageModalProps) => {
    modals.open({
      size: "100%",
      title: useTranslateModals("generate_trade_message.title", { count: input.items.length }),
      withCloseButton: false,
      children: <GenerateTradeMessageModal {...input} />,
    });
  };
  const OpenCreateRiven = () => {
    modals.open({
      title: useTranslateBasePrompt("create_riven.title"),
      size: "950px",
      children: (
        <CreateRiven
          onSubmit={async (data) => {
            await createMutation.mutateAsync({
              ...data,
              raw: data.wfm_weapon_url,
              rank: data.sub_type?.rank || 0,
            });
            modals.closeAll();
          }}
        />
      ),
    });
  };

  const OpenUpdateMultipleModal = (ids: number[]) => {
    let id = modals.open({
      size: "100%",
      withCloseButton: false,
      children: (
        <StockRivenUpdate
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

  const OpenFilterModal = (item: TauriTypes.StockRiven) => {
    const filter = item.filter || { enabled: false, attributes: [] };
    if (!filter.attributes) filter.attributes = item.attributes.map((x) => ({ positive: x.positive, url_name: x.url_name, is_required: false }));

    modals.open({
      title: useTranslateBasePrompt("update_filter.title"),
      size: "75vw",
      children: (
        <RivenFilter
          value={filter}
          onSubmit={async (data) => {
            await updateMutation.mutateAsync({ id: item.id, filter: data });
            modals.closeAll();
          }}
        />
      ),
    });
  };
  return {
    OpenMinimumPriceModal,
    OpenSellModal,
    OpenFilterModal,
    OpenInfoModal,
    OpenUpdateMultipleModal,
    OpenCreateRiven,
    OpenDeleteModal,
    OpenDeleteMultipleModal,
    OpenWTSModal,
  };
};
