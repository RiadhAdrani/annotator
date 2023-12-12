import { useContext, useState } from 'react';
import { TextAnnotationContext } from '../../contexts/TextAnnotation.context';
import { Button, Input, Skeleton, Text, Title } from '@mantine/core';

const TextAnnotationHeader = () => {
  const { annotation } = useContext(TextAnnotationContext);

  const [edit, setEdit] = useState(false);

  return (
    <div className="col mb-5 gap-2">
      {!annotation ? (
        <>
          <Skeleton height={20} />
          <Skeleton height={40} />
        </>
      ) : (
        <>
          <Text c={'gray'}>{annotation._id.$oid}</Text>
          {edit ? (
            <EditTextAnnotationName close={() => setEdit(false)} />
          ) : (
            <div className="row items-center gap-2">
              <Title>{annotation.title}</Title>
              <Button size="compact-xs" variant="light" onClick={() => setEdit(true)}>
                Edit
              </Button>
            </div>
          )}
        </>
      )}
    </div>
  );
};

const EditTextAnnotationName = ({ close }: { close: () => void }) => {
  const { annotation, updateAnnotation } = useContext(TextAnnotationContext);

  const [title, setName] = useState(annotation?.title ?? '');

  return (
    <div className="col gap-2">
      <Input size="lg" value={title} onChange={(e) => setName(e.currentTarget.value)} />
      <div className="row gap-2">
        <Button size="sm" variant="subtle" onClick={close}>
          Close
        </Button>
        <Button
          size="sm"
          onClick={() => {
            updateAnnotation({ title });
            close();
          }}
        >
          Save
        </Button>
      </div>
    </div>
  );
};

export default TextAnnotationHeader;
