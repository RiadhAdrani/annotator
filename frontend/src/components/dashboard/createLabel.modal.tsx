import { Button, Modal, ModalProps, Select, TextInput } from '@mantine/core';
import { useContext, useMemo, useState } from 'react';

import { TextAnnotation } from '../../types/annotations';
import AppContext from '../../contexts/App.context';

export interface CreateLabelModal extends ModalProps {
  annotation: TextAnnotation;
  onConfirm: (body: { name: string; color: string }) => void;
}

const CreateLabelModal = ({ opened, onClose, annotation, onConfirm }: CreateLabelModal) => {
  const { colors } = useContext(AppContext);

  const [color, setColor] = useState('');
  const [name, setName] = useState('');

  const colorOptions = useMemo(() => {
    const used = annotation.labels.map((it) => it.color);

    return Object.keys(colors).filter((it) => !used.includes(it));
  }, [annotation, colors]);

  return (
    <Modal opened={opened} centered onClose={onClose} title={'Create Label'}>
      <div className="col gap-3">
        <TextInput
          placeholder="Name"
          value={name}
          onChange={(e) => setName(e.currentTarget.value)}
        />
        <Select
          value={color}
          label="Color"
          placeholder="Label color"
          data={colorOptions}
          onChange={(v) => {
            if (v) setColor(v);
          }}
        />
        <div className="row gap-2 justify-end">
          <Button onClick={onClose}>Cancel</Button>
          <Button
            onClick={() => {
              setName('');
              setColor('');

              onClose();
              onConfirm({ name, color });
            }}
          >
            Create
          </Button>
        </div>
      </div>
    </Modal>
  );
};

export default CreateLabelModal;
