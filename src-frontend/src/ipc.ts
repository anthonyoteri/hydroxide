import { invoke } from "@tauri-apps/api";
import { deepFreeze } from "utils-min";

export async function ipc_invoke(
  method: string,
  params?: object
): Promise<any> {
  const response: any = await invoke(method, { params });

  if (response.error != null) {
    throw new Error(response.error);
  } else {
    return deepFreeze(response.result);
  }
}
