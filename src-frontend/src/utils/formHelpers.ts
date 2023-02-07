import { FormikErrors, FormikValues } from "formik";

type FormErrorContainer<T extends FormikValues> = {
  formErrors: FormikErrors<T> | undefined;
  error: Error;
};

export const parseErrors = <T extends FormikValues>(
  err: Error
): FormErrorContainer<T> => {
  const formErrors: FormikErrors<T> = {};

  return {
    formErrors,
    error: {
      ...err,
      /* errors: filtered.filter((e: ErrorItem | null) => e) as ErrorItem[], */
    },
  };
};

export type FormStatus =
  | "pending"
  | "saving"
  | "completed"
  | "cancelled"
  | "error";
