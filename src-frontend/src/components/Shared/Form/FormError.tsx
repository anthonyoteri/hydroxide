import { Alert } from "antd";
import React, { FC, useState } from "react";
import { CaretRightOutlined, CaretDownOutlined } from "@ant-design/icons";

export type FormErrorProps = {
  error: Error | null;
  summary?: string | React.ReactNode;
};

export const FormError: FC<FormErrorProps> = ({ error, summary }) => {
  const [expanded, setExpanded] = useState(false);
  if (!error) return null;

  const getMessage = (err: Error) => {
    return err;
  };

  const message = getMessage(error);

  const content = summary ? (
    <>
      <span
        onClick={() => setExpanded((exp) => !exp)}
        style={{ cursor: "pointer" }}
      >
        {expanded ? <CaretDownOutlined /> : <CaretRightOutlined />}
      </span>{" "}
      {summary}
    </>
  ) : (
    message
  );

  return message ? (
    <div style={{ marginTop: "12px" }}>
      <Alert type="error" message={content.toString()} />
    </div>
  ) : null;
};
