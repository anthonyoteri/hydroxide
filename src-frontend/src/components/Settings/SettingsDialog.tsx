import React, { FC } from "react";
import { Formik } from "formik";
import { SettingsForm } from "./SettingsForm";

interface Props {
  hideAdvanced: boolean;
}

export const SettingsDialog: FC<Props> = (props: Props) => {
  const { hideAdvanced } = props;

  return (
    <Formik initialValues={{}} onSubmit={() => console.log("TODO: Submit")}>
      {(props) => (
        <SettingsForm
          formik={props}
          isLoading={false}
          hideAdvanced={hideAdvanced}
        />
      )}
    </Formik>
  );
};
