import { useContext } from 'react';
import TextAnnotationHeader from '../../components/dashboard/text.header';
import TextAnnotationLabels from '../../components/dashboard/text.labels';
import {
  TextAnnotationContext,
  TextAnnotationProvider,
} from '../../contexts/TextAnnotation.context';
import { Skeleton } from '@mantine/core';
import TextAnnotationParagraph from '../../components/dashboard/text.paragraph';

const Page = () => {
  const { annotation, paragraphs } = useContext(TextAnnotationContext);

  return (
    <>
      <TextAnnotationHeader />
      <TextAnnotationLabels />
      <div className="col gap-2">
        {!annotation ? (
          <>
            <Skeleton height={75} />
            <Skeleton height={100} />
            <Skeleton height={150} />
            <Skeleton height={75} />
            <Skeleton height={100} />
          </>
        ) : (
          <>
            {paragraphs.map((p, key) => (
              <TextAnnotationParagraph key={key} item={p} />
            ))}
          </>
        )}
      </div>
    </>
  );
};

const TextAnnotationPage = () => {
  return (
    <TextAnnotationProvider>
      <Page />
    </TextAnnotationProvider>
  );
};

export default TextAnnotationPage;
