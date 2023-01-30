import { FC, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { useAppDispatch, useAppSelector } from "../../hooks";
import { AuthStatus, checkAuth } from "../../store/auth";
import { ApplicationState } from "../../store/rootReducer";
import Loading from "./Loading";

export const WaitForAuthCheck: FC<React.PropsWithChildren<{}>> = ({
  children,
}) => {
  const { t } = useTranslation();
  const dispatch = useAppDispatch();
  const { status: authStatus } = useAppSelector(
    (state: ApplicationState) => state.auth
  );

  useEffect(() => {
    if (authStatus === AuthStatus.WAITING) {
      dispatch(checkAuth());
    }
  }, [dispatch, authStatus]);

  if (authStatus === AuthStatus.WAITING || authStatus === AuthStatus.PENDING) {
    return <Loading message={t("common.authenticating")} />;
  }

  return <>{children}</>;
};
