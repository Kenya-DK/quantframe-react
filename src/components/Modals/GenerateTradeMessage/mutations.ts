import { TauriTypes } from "$types";
import api from "@api/index";
import { createGenericMutation, MutationHooks } from "@utils/genericMutation.helper";

export const useMutations = ({ refetchQueries, setLoadingRows }: MutationHooks) => {
  const hooks = { refetchQueries, setLoadingRows };

  const createMutation = createGenericMutation(
    {
      mutationFn: async (data: { settings: TauriTypes.Settings; template: TauriTypes.SaveTemplateSetting }) => {
        const { settings, template } = data;
        const otherTemplates = settings.generate_trade_message.templates.filter((t) => t.name !== template.name) || [];
        const updatedTemplates = [...otherTemplates, template];
        const updatedSettings = {
          ...settings,
          generate_trade_message: {
            ...settings.generate_trade_message,
            templates: updatedTemplates,
          },
        };
        return await api.app.updateSettings(updatedSettings);
      },
      successKey: "create_trade_msg_template",
      errorKey: "create_trade_msg_template",
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );

  const deleteMutation = createGenericMutation(
    {
      mutationFn: async (data: { settings: TauriTypes.Settings; name: string }) => {
        const { settings, name } = data;
        const updatedTemplates = settings.generate_trade_message.templates.filter((t) => t.name !== name) || [];
        const updatedSettings = {
          ...settings,
          generate_trade_message: {
            ...settings.generate_trade_message,
            templates: updatedTemplates,
          },
        };
        return await api.app.updateSettings(updatedSettings);
      },
      successKey: "delete_trade_msg_template",
      errorKey: "delete_trade_msg_template",
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );

  return {
    createMutation,
    deleteMutation,
  };
};
