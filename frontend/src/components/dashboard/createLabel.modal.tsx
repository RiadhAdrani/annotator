import { Button, Modal, ModalProps, TextInput } from '@mantine/core';
import { useContext, useMemo, useState } from 'react';

import { TextAnnotationContext } from '../../contexts/TextAnnotation.context';
import LabelColorPicker from './labelColorPicker';

export interface CreateLabelModal extends ModalProps {}

const CreateLabelModal = ({ opened, onClose }: CreateLabelModal) => {
  const { createLabel, annotation } = useContext(TextAnnotationContext);

  const [color, setColor] = useState('');
  const [name, setName] = useState('');

  const used = useMemo(() => {
    if (!annotation) return [];

    return annotation.labels.map((it) => it.color);
  }, [annotation]);

  return (
    <Modal opened={opened} centered onClose={onClose} title={'Create Label'}>
      <div className="col gap-3">
        <TextInput
          placeholder="Name"
          value={name}
          onChange={(e) => setName(e.currentTarget.value)}
        />
        <LabelColorPicker filter={used} selected={color} onSelected={setColor} />
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
