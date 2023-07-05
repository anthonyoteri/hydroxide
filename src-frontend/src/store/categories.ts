import { createSelector, createSlice, PayloadAction } from "@reduxjs/toolkit";
import { category_fmc } from "../model";
import { Category } from "../bindings/Category";
import { AppThunk } from "./index";
import { ApplicationState } from "./rootReducer";
import { CategoryDraft, ModelMutateResultData } from "../bindings";

interface CategoryState {
  allIds: string[];
  byId: Record<string, Category>;
}

const initialState: CategoryState = {
  allIds: [],
  byId: {},
};

export const categoryReducer = createSlice({
  name: "categories",
  initialState,
  reducers: {
    fetchSuccess(state, action: PayloadAction<Category[]>) {
      state.allIds = action.payload.map((category) => category.id);
      state.byId = {};
      action.payload.forEach((category) => {
        state.byId[category.id] = category;
      });
    },
    fetchFail(state, action: PayloadAction<any>) {},
  },
});

export const { fetchSuccess, fetchFail } = categoryReducer.actions;

export default categoryReducer.reducer;

export const fetchCategories =
  (): AppThunk<Promise<void>> => async (dispatch) => {
    try {
      const categories = await category_fmc.list();
      dispatch(fetchSuccess(categories));
    } catch (err) {
      console.error(err);
      dispatch(fetchFail(err));
      throw err;
    }
  };

export const deleteCategory =
  (id: string): AppThunk<Promise<void>> =>
  async (dispatch) => {
    try {
      await category_fmc.delete(id);
      dispatch(fetchCategories());
    } catch (err) {
      throw err;
    }
  };

export const createCategory =
  (body: CategoryDraft): AppThunk<Promise<ModelMutateResultData>> =>
  async (dispatch) => {
    try {
      const newId = await category_fmc.create(body);
      await dispatch(fetchCategories());
      return newId;
    } catch (err) {
      throw err;
    }
  };

export const patchCategory =
  (id: string, body: CategoryDraft): AppThunk<Promise<void>> =>
  async (dispatch) => {
    try {
      await category_fmc.update(id, body);
      return dispatch(fetchCategories());
    } catch (err) {
      throw err;
    }
  };

export const selectAllCategories = createSelector(
  (state: ApplicationState) => state.categories,
  ({ allIds, byId }) => allIds.map((id) => byId[id]),
);
