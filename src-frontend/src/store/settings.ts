import { settings_fmc } from "../model";

export const uploadConfiguration = (data: any) => {
  return settings_fmc.importConfiguration(data);
};
