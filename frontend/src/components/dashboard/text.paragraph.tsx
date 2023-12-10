import { useContext, useMemo } from 'react';
import { Paragraph, TextAnnotationContext } from '../../contexts/TextAnnotation.context';
import TextAnnotationWord from './text.word';
import AppContext from '../../contexts/App.context';
import { Card } from '@mantine/core';
import { changeColorOpacity } from '@riadh-adrani/color-utils';

export interface TextAnnotationParagraphProps {
  item: Paragraph;
}

const TextAnnotationParagraph = ({ item }: TextAnnotationParagraphProps) => {
  const { annotation, selectedLabel, cursorHint, isHighlighting, cancelCursor, finishCursor } =
    useContext(TextAnnotationContext);
  const { colors } = useContext(AppContext);

  const highlightColor = useMemo(() => {
    const label = annotation?.labels.find((it) => it._id.$oid === selectedLabel);

    if (!label) return '#ffffff';

    return colors[label.color];
  }, [selectedLabel, colors, annotation]);

  return (
    <>
      <Card shadow="xs">
        <div className="text-0.9em">{cursorHint}</div>
      </Card>
      <Card
        shadow={isHighlighting ? 'lg' : 'xs'}
        bg={isHighlighting ? changeColorOpacity(highlightColor, 0.05) : ''}
      >
        <div
          style={
            { '--highlight-color': changeColorOpacity(highlightColor, 0.15) } as React.CSSProperties
          }
          className="paragraph row flex-wrap whitespace-pre p-x-5"
          onMouseLeave={cancelCursor}
          onMouseUp={finishCursor}
        >
          {item.words.map((word, key) => (
            <TextAnnotationWord item={word} key={key} />
          ))}
        </div>
      </Card>
    </>
  );
};

export default TextAnnotationParagraph;
