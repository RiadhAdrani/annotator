import { Word } from '../../contexts/TextAnnotation.context';
import TextAnnotationCharacter from './text.character';

export interface TextWordProps {
  item: Word;
}

const TextAnnotationWord = ({ item }: TextWordProps) => {
  return (
    <div className="word row whitespace-pre">
      {item.characters.map((word) => (
        <TextAnnotationCharacter item={word} key={`${word.char}@${word.index}`} />
      ))}
    </div>
  );
};

export default TextAnnotationWord;
