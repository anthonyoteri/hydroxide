import {
  ModelMutateResultData,
  Category,
  CategoryDraft,
  Project,
  ProjectDraft,
  TimeRecord,
  TimeRecordDraft,
} from "../bindings";
import { ipc_invoke } from "../ipc";
import { ensure_ModelMutateResultData } from "../bindings/type_asserts";
import moment from "moment";

class BaseFmc<M, C, U> {
  #cmd_suffix: string;
  get cmd_suffix() {
    return this.#cmd_suffix;
  }

  constructor(cmd_suffix: string) {
    this.#cmd_suffix = cmd_suffix;
  }

  serialize(obj: M | C | U): string {
    return JSON.stringify(obj, (key, value) => {
      console.log(`key=${key}, value=${value}`);
      if (value instanceof Date || moment.isMoment(value)) {
        return value.toISOString();
      }
      return value;
    });
  }

  async get(id: string): Promise<M> {
    console.log(`Get::${this.#cmd_suffix}`, id);
    return ipc_invoke(`get_${this.#cmd_suffix}`, { id }).then(
      (res) => res.data
    );
  }

  async create(data: C): Promise<ModelMutateResultData> {
    console.log(`Create::${this.#cmd_suffix}`, data);
    const serialized = this.serialize(data);
    return ipc_invoke(`create_${this.#cmd_suffix}`, { data: JSON.parse(serialized) }).then((res) => {
      return ensure_ModelMutateResultData(res.data);
    });
  }

  async update(id: string, data: U): Promise<ModelMutateResultData> {
    console.log(`Update::${this.#cmd_suffix}`, id, data);
    const serialized = this.serialize(data);
    return ipc_invoke(`update_${this.#cmd_suffix}`, { id, data: JSON.parse(serialized) }).then(
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

class TimeRecordFmc extends BaseFmc<TimeRecord, TimeRecordDraft, TimeRecordDraft> {
  constructor() {
    super("time_record");
  }

  async list(): Promise<TimeRecord[]> {
    return ipc_invoke(`list_time_records`, {}).then((res) => res.data);
  }
}

export const time_record_fmc = new TimeRecordFmc();