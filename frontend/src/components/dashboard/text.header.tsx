import { useContext } from 'react';
import { TextAnnotationContext } from '../../contexts/TextAnnotation.context';
import { Skeleton, Text, Title } from '@mantine/core';

const TextAnnotationHeader = () => {
  const { annotation } = useContext(TextAnnotationContext);

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
          <Title>{annotation.title}</Title>
        </>
      )}
    </div>
  );
};

export default TextAnnotationHeader;
