import React from "react";
import { render, act } from "@testing-library/react";
import App from "./App";
import { Provider } from "react-redux";
import { store } from "./store";

test("renders app without crashing", () => {
  act(() => {
    render(
      <Provider store={store}>
        <App />
      </Provider>,
    );
  });
});
