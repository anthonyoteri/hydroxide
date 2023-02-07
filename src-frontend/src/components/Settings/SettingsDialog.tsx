import React, { FC, useState } from "react";
import { Formik } from "formik";
import { useSelector, useDispatch } from "react-redux";
import { useTranslation } from "react-i18next";
import { message, notification } from "antd";
import * as actions from "../../store/settings";
import { SettingsForm } from "./SettingsForm";
import { AppDispatch } from "../../store";

interface Props {
  hideAdvanced: boolean;
}

export const SettingsDialog: FC<Props> = (props: Props) => {
  const { t } = useTranslation();
  const dispatch = useDispatch<AppDispatch>();
  const { hideAdvanced } = props;

  const [isLoading, setIsLoading] = useState(false);

  return (
    <Formik initialValues={{}} onSubmit={() => console.log("TODO: Submit")}>
      {(props) => (
        <SettingsForm
          formik={props}
          isLoading={isLoading}
          hideAdvanced={hideAdvanced}
        />
      )}
    </Formik>
  );
};
