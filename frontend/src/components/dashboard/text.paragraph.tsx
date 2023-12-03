import { useContext, useMemo } from 'react';
import { Paragraph, TextAnnotationContext } from '../../contexts/TextAnnotation.context';
import TextAnnotationWord from './text.word';
import AppContext from '../../contexts/App.context';

export interface TextAnnotationParagraphProps {
  item: Paragraph;
}

const TextAnnotationParagraph = ({ item }: TextAnnotationParagraphProps) => {
  const { annotation, selectedLabel } = useContext(TextAnnotationContext);
  const { colors } = useContext(AppContext);

  const highlightColor = useMemo(() => {
    const label = annotation?.labels.find((it) => it._id.$oid === selectedLabel);

    if (!label) return 'transparent';

    return colors[label.color];
  }, [selectedLabel, colors, annotation]);

  return (
    <div
      style={{ '--highlight-color': highlightColor } as React.CSSProperties}
      className="paragraph row flex-wrap whitespace-pre"
    >
      {item.words.map((word, key) => (
        <TextAnnotationWord item={word} key={key} />
      ))}
    </div>
  );
};

export default TextAnnotationParagraph;
