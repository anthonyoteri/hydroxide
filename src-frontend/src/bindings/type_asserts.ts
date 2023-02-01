import { ModelMutateResultData } from "./ModelMutateResultData";

export function ensure_ModelMutateResultData(obj: any): ModelMutateResultData {
  const keys = Object.keys(obj);
  if (keys.length != 1 || keys[0] != "id" || typeof obj["id"] !== "string") {
    throw new Error("assert ensure_ModelMutateResultData failed {obj}");
  }
  return obj;
}
