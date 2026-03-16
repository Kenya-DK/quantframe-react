import { modals } from "@mantine/modals";
import { TauriTypes } from "$types";
import { useTranslateCommon } from "@hooks/useTranslate.hook";

interface ModalHooks {
  createMutation: {
    mutateAsync: (data: TauriTypes.CreateStockRiven) => Promise<any>;
  };
}

export const useModals = ({ createMutation }: ModalHooks) => {
  const OpenBoughtModal = (order: TauriTypes.CreateStockRiven) => {
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
          if (!order) return;
          const { bought } = data;
          await createMutation.mutateAsync({ ...order, bought });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };

  return {
    OpenBoughtModal,
  };
};
