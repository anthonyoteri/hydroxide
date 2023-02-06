import React, { FC, useEffect, useState } from "react";
import { getVersion, getName } from "@tauri-apps/api/app";

import { BranchesOutlined } from "@ant-design/icons";

export const AppVersion: FC<{}> = () => {
  const [appName, setAppName] = useState<string | undefined>(undefined);
  const [appVersion, setAppVersion] = useState<string | undefined>(undefined);

  useEffect(() => {
    const fetchData = async () => {
      setAppVersion(await getVersion());
      setAppName(await getName());
    };
    fetchData();
  });

  return (
    <div data-testid="app_version" className="app-version">
      <div className="app-version-number">
        <BranchesOutlined />
        {appName} V:{appVersion}
      </div>
    </div>
  );
};
