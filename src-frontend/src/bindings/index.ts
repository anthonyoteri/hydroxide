//! Re-export all bindings for import convenience
import { Category } from "./Category";
import { Project } from "./Project";

export * from "./Category";

export interface CategoryDraft extends Omit<Category, "id" | "ctime"> {}

export * from "./Project";

export interface ProjectDraft extends Omit<Project, "id" | "ctime"> {}

export * from "./ModelMutateResultData";
