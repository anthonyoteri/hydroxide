//! Re-export all bindings for import convenience
import { Category } from "./Category";
import { Project } from "./Project";
import { TimeRecord } from "./TimeRecord";

export * from "./Category";

export interface CategoryDraft extends Omit<Category, "id" | "ctime"> { }

export * from "./Project";

export interface ProjectDraft extends Omit<Project, "id" | "ctime"> { }

export * from "./ModelMutateResultData";

export interface TimeRecordDraft
    extends Omit<
        TimeRecord,
        "id" | "start_time" | "stop_time" | "approved"
    > {
    start_time?: Date;
    stop_time?: Date;
    approved?: boolean;
}

export * from "./TimeRecord";
