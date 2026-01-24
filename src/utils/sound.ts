export const CUSTOM_SOUND_PREFIX = "custom:";
export const FALLBACK_SOUND = "cat_meow.mp3";

export const isCustomSound = (fileName: string) => fileName.startsWith(CUSTOM_SOUND_PREFIX);

export const toCustomSoundValue = (fileName: string) => `${CUSTOM_SOUND_PREFIX}${fileName}`;

export const stripCustomSoundPrefix = (fileName: string) =>
  isCustomSound(fileName) ? fileName.slice(CUSTOM_SOUND_PREFIX.length) : fileName;
