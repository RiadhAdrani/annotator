import { Button, Modal } from '@mantine/core';
import { PropsWithChildren } from 'react';

interface ConfirmModalProps {
  opened: boolean;
  onClose: () => void;
  onAccept: () => void;
  title?: string;
}

const ConfirmModal = ({
  onAccept,
  children,
  opened,
  onClose,
  title,
}: PropsWithChildren<ConfirmModalProps>) => {
  return (
    <Modal opened={opened} centered onClose={onClose} title={title}>
      <Modal.Body className="col gap-2">
        <div>{children}</div>
        <div className="row justify-end gap-2">
          <Button variant="subtle" onClick={onClose}>
            Cancel
          </Button>
          <Button onClick={onAccept} color="red">
            Confirm
          </Button>
        </div>
      </Modal.Body>
    </Modal>
  );
};

export default ConfirmModal;
