import { Button, Modal, ModalProps, Select, TextInput } from '@mantine/core';
import { useContext, useMemo, useState } from 'react';

import AppContext from '../../contexts/App.context';
import { TextAnnotationContext } from '../../contexts/TextAnnotation.context';

export interface CreateLabelModal extends ModalProps {}

const CreateLabelModal = ({ opened, onClose }: CreateLabelModal) => {
  const { colors } = useContext(AppContext);
  const { createLabel, annotation } = useContext(TextAnnotationContext);

  const [color, setColor] = useState('');
  const [name, setName] = useState('');

  const colorOptions = useMemo(() => {
    if (!annotation) return [];

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

              createLabel({ name, color });
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
