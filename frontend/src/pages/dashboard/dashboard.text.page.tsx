import { Fragment, useContext } from 'react';
import TextAnnotationHeader from '../../components/dashboard/text.header';
import TextAnnotationLabels from '../../components/dashboard/text.labels';
import {
  TextAnnotationContext,
  TextAnnotationProvider,
} from '../../contexts/TextAnnotation.context';
import { Card, Divider, Skeleton } from '@mantine/core';
import TextAnnotationParagraph from '../../components/dashboard/text.paragraph';

const Page = () => {
  const { annotation, paragraphs, cursorHint } = useContext(TextAnnotationContext);

  return (
    <>
      <TextAnnotationHeader />
      <Divider className="m-y-5" />
      <TextAnnotationLabels />
      <Divider className="m-y-5" />
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
            <Card shadow="xs">
              <div className="text-0.9em">{cursorHint}</div>
            </Card>
            {paragraphs.map((p, key) => (
              <Fragment key={key}>
                {p.words.length > 0 && <TextAnnotationParagraph item={p} />}
              </Fragment>
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
