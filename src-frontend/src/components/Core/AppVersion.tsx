import { FC, useEffect, useState } from "react";
import {
  BranchesOutlined,
  CalendarOutlined,
  GithubOutlined,
} from "@ant-design/icons";
import moment from "moment";

import * as api from "../../api/About/About";

export const AppVersion: FC<{}> = () => {
  const [appVersion, setAppVersion] = useState<string | undefined>(undefined);
  const [revision, setRevision] = useState<string | undefined>(undefined);
  const [buildDate, setBuildDate] = useState<Date | undefined>(undefined);

  useEffect(() => {
    const fetchData = async () => {
      const appInfo = await api.getAppInfo();
      setAppVersion(appInfo?.app_version);
      setRevision(appInfo?.revision);
      setBuildDate(appInfo?.build_date);
    };
    fetchData();
  });

  return (
    <div data-testid="app_version" className="app-version">
      <div className="app-version-number">
        <BranchesOutlined /> V:{appVersion}
        <span className="app-version-revision">
          {"  "}
          <GithubOutlined /> {revision}
        </span>
      </div>
      <div className="app-version-date">
        <CalendarOutlined /> {moment(buildDate).format("LL")}
      </div>
    </div>
  );
};
