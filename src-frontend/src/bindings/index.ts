//! Re-export all bindings for import convenience
import { Category } from "./Category";
import { Project } from "./Project";
import { TimeRecord } from "./TimeRecord";

import moment from "moment";

export * from "./Category";

export interface CategoryDraft extends Omit<Category, "id" | "ctime"> { }

export * from "./Project";

export interface ProjectDraft extends Omit<Project, "id" | "ctime"> { }

export * from "./ModelMutateResultData";

export interface TimeRecordDraft
    extends Omit<
        TimeRecord,
        "id" | "start_time" | "stop_time"
    > {
    start_time?: Date;
    stop_time?: Date;
}

export * from "./TimeRecord";

export function fromJSON<T>(json: any): T {
    let result = {} as T;
    const data = json as T;
  
    for (const key in data) {
      let value: any = data[key];
  
      switch (typeof value) {
        case 'string':
          if (moment(value, moment.ISO_8601).isValid()) {
            value = moment(value).toDate();
          }
          break;
      }
  
      result[key] = value;
    }
  
    return result;
  }