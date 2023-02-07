import { useState } from "react";

type ModalFormState =
  | "pending"
  | "saving"
  | "completed"
  | "cancelled"
  | "error";

export const useModalForm = ({
  onCancel,
  onClose,
  onComplete,
}: {
  // these will most likely not be needed, a single onClose-function should cover both cases
  onCancel?: Function;
  onComplete?: Function;

  onClose?: () => void;
}) => {
  const [status, setStatus] = useState<ModalFormState>("pending");
  const [error, setError] = useState<Error | null>(null);

  const handleAfterClose = () => {
    if (status === "cancelled") {
      onCancel && onCancel();
    }
    if (status === "completed") {
      onComplete && onComplete();
    }

    onClose && onClose();
  };

  return {
    handleAfterClose,
    isVisible: ["pending", "saving", "error"].includes(status),
    isSaving: ["saving"].includes(status),
    setStatus,
    status,
    error,
    setError,
  };
};

export default useModalForm;
