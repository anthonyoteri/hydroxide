import { UploadOutlined } from "@ant-design/icons";
import { Button } from "antd";
import React, { FC } from "react";
import { useTranslation } from "react-i18next";
import { open } from "@tauri-apps/api/dialog";
import { readTextFile } from "@tauri-apps/api/fs";
import { uploadConfiguration } from "../../store/settings";

type Props = {
  disabled: boolean;
};

export const SettingsRestore: FC<Props> = ({ disabled }) => {
  const { t } = useTranslation();

  const restore = async () => {
    try {
      const selectedPath = await open({
        multiple: false,
        title: "Import data",
      });

      if (!selectedPath) return;
      const contents = await readTextFile(selectedPath as string);
      const body = JSON.parse(contents);
      console.log("before");
      await uploadConfiguration(body);
      console.log("After");
      window.location.reload();
    } catch (err) {
      console.log(err);
    }
  }

  return (
    <div>
      <Button onClick={restore}>
        <UploadOutlined /> {t("settings.restore.showUploadButton")}
      </Button>
    </div>
  );
};
