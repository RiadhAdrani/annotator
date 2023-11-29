export interface ObjectId {
  $oid: string;
}

export interface Base {
  _id: ObjectId;
}

export interface ModalProps {
  opened: boolean;
  onClose: () => void;
}
