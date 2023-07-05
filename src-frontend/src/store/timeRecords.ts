import { createSlice, createSelector, PayloadAction } from "@reduxjs/toolkit";
import {
  ModelMutateResultData,
  TimeRecord,
  TimeRecordDraft,
} from "../bindings";
import { time_record_fmc } from "../model";
import { AppThunk } from "./index";
import { ApplicationState } from "../store/rootReducer";
import moment from "moment";

interface TimeRecordState {
  allIds: string[];
  byId: Record<string, TimeRecord>;
}

const initialState: TimeRecordState = {
  allIds: [],
  byId: {},
};

export const timeRecordReducer = createSlice({
  name: "records",
  initialState,
  reducers: {
    fetchSuccess(state, action: PayloadAction<TimeRecord[]>) {
      state.allIds = action.payload.map((record) => record.id);
      state.byId = {};
      action.payload.forEach((record) => {
        state.byId[record.id] = record;
      });
    },
    fetchFail(state, action: PayloadAction<any>) {},
  },
});

export const { fetchSuccess, fetchFail } = timeRecordReducer.actions;

export default timeRecordReducer.reducer;

export const fetchRecords = (): AppThunk<Promise<void>> => async (dispatch) => {
  try {
    const records = await time_record_fmc.list();
    console.log(records);
    dispatch(fetchSuccess(records));
  } catch (err) {
    console.error(err);
    dispatch(fetchFail(err));
    throw err;
  }
};

export const deleteRecord =
  (id: string): AppThunk<Promise<void>> =>
  async (dispatch) => {
    try {
      await time_record_fmc.delete(id);
      dispatch(fetchRecords());
    } catch (err) {
      throw err;
    }
  };

export const createRecord =
  (body: TimeRecordDraft): AppThunk<Promise<ModelMutateResultData>> =>
  async (dispatch) => {
    try {
      const newId = await time_record_fmc.create(body);
      await dispatch(fetchRecords());
      return newId;
    } catch (err) {
      throw err;
    }
  };

export const patchRecord =
  (id: string, body: TimeRecordDraft): AppThunk<Promise<void>> =>
  async (dispatch) => {
    try {
      await time_record_fmc.update(id, body);
      return dispatch(fetchRecords());
    } catch (err) {
      throw err;
    }
  };

export const selectAllRecords = createSelector(
  (state: ApplicationState) => state.records.allIds,
  (state: ApplicationState) => state.records.byId,
  (allIds, byId) => allIds.map((id) => byId[id]),
);

export const selectRecordsForWeek = createSelector(
  [selectAllRecords, (state: ApplicationState, week: number) => week],
  (records, week) => {
    return records.filter(
      (r: TimeRecord) => moment(r.start_time).isoWeek() === week,
    );
  },
);
