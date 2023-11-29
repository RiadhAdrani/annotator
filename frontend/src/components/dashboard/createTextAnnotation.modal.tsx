import { Button, Modal, ModalProps, TextInput, Textarea } from '@mantine/core';
import { useCallback, useContext, useState } from 'react';

import { DashboardContext } from '../../contexts/Dashboard.context';

const CreateTextAnnotationModal = ({ opened, onClose }: ModalProps) => {
  const { createTextAnnotation } = useContext(DashboardContext);

  const [content, setContent] = useState('');
  const [title, setTitle] = useState('');

  const onSubmit = useCallback(
    async () =>
      createTextAnnotation({ content, title }).then(() => {
        setContent('');
        setTitle('');

        onClose();
      }),
    [content, title, createTextAnnotation, onClose]
  );

  return (
    <Modal opened={opened} centered onClose={onClose} title={'Create Text Annotation'}>
      <div className="col gap-3">
        <TextInput
          placeholder="Title"
          value={title}
          onChange={(e) => setTitle(e.currentTarget.value)}
        />
        <Textarea
          placeholder="Content"
          value={content}
          onChange={(e) => setContent(e.currentTarget.value)}
        />
        <div className="row gap-2 justify-end">
          <Button onClick={onClose}>Cancel</Button>
          <Button onClick={onSubmit}>Create</Button>
        </div>
      </div>
    </Modal>
  );
};

export default CreateTextAnnotationModal;
