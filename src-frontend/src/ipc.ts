import { invoke } from "@tauri-apps/api";
import { deepFreeze } from "utils-min";

export async function ipc_invoke(
  method: string,
  params?: object
): Promise<any> {
  console.log("ipc_invoke: ", method, params);
  const response: any = await invoke(method, { params });
  console.log("ipc_invoke: Returned, ", response);

  if (response.error != null) {
    console.log("ERROR - ipc-invoke - ipc_invoke error", response);
    throw new Error(response.error);
  } else {
    console.log("Returning ", response.result);
    return deepFreeze(response.result);
  }
}
