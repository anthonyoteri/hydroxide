import {
  ModelMutateResultData,
  Category,
  CategoryDraft,
  Project,
  ProjectDraft,
} from "../bindings";
import { ipc_invoke } from "../ipc";
import { ensure_ModelMutateResultData } from "../bindings/type_asserts";

class BaseFmc<M, C, U> {
  #cmd_suffix: string;
  get cmd_suffix() {
    return this.#cmd_suffix;
  }

  constructor(cmd_suffix: string) {
    this.#cmd_suffix = cmd_suffix;
  }

  async get(id: string): Promise<M> {
    console.log(`Get::${this.#cmd_suffix}`, id);
    return ipc_invoke(`get_${this.#cmd_suffix}`, { id }).then(
      (res) => res.data
    );
  }

  async create(data: C): Promise<ModelMutateResultData> {
    console.log(`Create::${this.#cmd_suffix}`, data);
    return ipc_invoke(`create_${this.#cmd_suffix}`, { data }).then((res) => {
      return ensure_ModelMutateResultData(res.data);
    });
  }

  async update(id: string, data: U): Promise<ModelMutateResultData> {
    console.log(`Update::${this.#cmd_suffix}`, id, data);
    return ipc_invoke(`update_${this.#cmd_suffix}`, { id, data }).then(
      (res) => {
        return ensure_ModelMutateResultData(res.data);
      }
    );
  }

  async delete(id: string): Promise<ModelMutateResultData> {
    console.log(`Delete::${this.#cmd_suffix}`, id);
    return ipc_invoke(`delete_${this.#cmd_suffix}`, { id }).then((res) => {
      return ensure_ModelMutateResultData(res.data);
    });
  }
}

class CategoryFmc extends BaseFmc<Category, CategoryDraft, CategoryDraft> {
  constructor() {
    super("category");
  }

  async list(): Promise<Category[]> {
    console.log(`List::category`);
    return ipc_invoke(`list_categories`, {}).then((res) => res.data);
  }
}

export const category_fmc = new CategoryFmc();

class ProjectFmc extends BaseFmc<Project, ProjectDraft, ProjectDraft> {
  constructor() {
    super("project");
  }

  async list(): Promise<Project[]> {
    return ipc_invoke(`list_projects`, {}).then((res) => res.data);
  }
}

export const project_fmc = new ProjectFmc();
