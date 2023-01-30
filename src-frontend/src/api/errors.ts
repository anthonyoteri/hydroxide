export type ErrorItem = {
  message: string;
  code: string;
  field?: string;
};

type ServerError = {
  type: "server";
  errors: ErrorItem[];
  status: number;
};

type NetworkError = {
  type: "network";
  error: string;
  errors?: ErrorItem[];
};

type UnknownError = {
  type: "unknown";
  error: string;
  errors?: ErrorItem[];
};

export type ApiError = ServerError | NetworkError | UnknownError;

export const apiErrorMessage = (error: ApiError) => {
  if (error.type === "server") {
    return `${error.errors.map((e) => e.message).join("\n")}`;
  } else if (error.type === "network") {
    return error.error;
  }
  return error.error;
};
