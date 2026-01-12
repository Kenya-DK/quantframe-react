import { TauriClient } from "..";
import { TauriTypes } from "$types";

export class SoundModule {
    constructor(private readonly client: TauriClient) { }

    getCustomSounds(): Promise<TauriTypes.CustomSound[]> {
        return this.client.sendInvoke<TauriTypes.CustomSound[]>("sound_get_custom_sounds");
    }

    addCustomSound(name: string, filePath: string): Promise<TauriTypes.CustomSound[]> {
        return this.client.sendInvoke<TauriTypes.CustomSound[]>("sound_add_custom_sound", { name, filePath });
    }

    deleteCustomSound(fileName: string): Promise<TauriTypes.CustomSound[]> {
        return this.client.sendInvoke<TauriTypes.CustomSound[]>("sound_delete_custom_sound", { fileName });
    }
}
