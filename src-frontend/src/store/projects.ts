import { createSelector, createSlice, PayloadAction } from "@reduxjs/toolkit";
import { Project, ProjectDraft, ModelMutateResultData } from "../bindings";
import { project_fmc } from "../model";
import { AppThunk } from "./index";
import { ApplicationState } from "./rootReducer";

interface ProjectState {
  allIds: string[];
  byId: Record<string, Project>;
}

const initialState: ProjectState = {
  allIds: [],
  byId: {},
};

export const projectReducer = createSlice({
  name: "projects",
  initialState,
  reducers: {
    fetchSuccess(state, action: PayloadAction<Project[]>) {
      state.allIds = action.payload.map((Project) => Project.id);
      state.byId = {};
      action.payload.forEach((Project) => {
        state.byId[Project.id] = Project;
      });
    },
    fetchFail(state, action: PayloadAction<any>) {},
  },
});

export const { fetchSuccess, fetchFail } = projectReducer.actions;

export default projectReducer.reducer;

export const fetchProjects =
  (): AppThunk<Promise<void>> => async (dispatch) => {
    try {
      const projects = await project_fmc.list();
      dispatch(fetchSuccess(projects));
    } catch (err) {
      console.error(err);
      dispatch(fetchFail(err));
      throw err;
    }
  };

export const deleteProject =
  (id: string): AppThunk<Promise<void>> =>
  async (dispatch) => {
    try {
      await project_fmc.delete(id);
      dispatch(fetchProjects());
    } catch (err) {
      throw err;
    }
  };

export const createProject =
  (body: ProjectDraft): AppThunk<Promise<ModelMutateResultData>> =>
  async (dispatch) => {
    try {
      let newProject = await project_fmc.create(body);
      await dispatch(fetchProjects());
      return newProject;
    } catch (err) {
      throw err;
    }
  };

export const patchProject =
  (id: string, body: ProjectDraft): AppThunk<Promise<void>> =>
  async (dispatch) => {
    try {
      await project_fmc.update(id, body);
      return dispatch(fetchProjects());
    } catch (err) {
      throw err;
    }
  };

export const selectAllProjects = createSelector(
  (state: ApplicationState) => state.projects.allIds,
  (state: ApplicationState) => state.projects.byId,
  (allIds, byId) => allIds.map((id) => byId[id])
);

export const selectProjectsForCategory = createSelector(
  [
    selectAllProjects,
    (state: ApplicationState, category: string | undefined) => category,
  ],
  (projects, category) =>
    category
      ? projects.filter((p: Project) => p.category === category)
      : projects
);
