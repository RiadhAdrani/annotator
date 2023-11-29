import { Paragraph } from '../../contexts/TextAnnotation.context';
import TextAnnotationWord from './text.word';

export interface TextAnnotationParagraphProps {
  item: Paragraph;
}

const TextAnnotationParagraph = ({ item }: TextAnnotationParagraphProps) => {
  return (
    <div className="paragraph row flex-wrap gap-2">
      {item.words.map((word, key) => (
        <TextAnnotationWord item={word} key={key} />
      ))}
    </div>
  );
};

export default TextAnnotationParagraph;
