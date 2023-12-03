import { useContext, useMemo } from 'react';
import { Character, TextAnnotationContext } from '../../contexts/TextAnnotation.context';
import AppContext from '../../contexts/App.context';

export interface TextCharacterProps {
  item: Character;
}

const TextAnnotationCharacter = ({ item }: TextCharacterProps) => {
  const { updateCursor } = useContext(TextAnnotationContext);
  const { colors } = useContext(AppContext);

  const color = useMemo(() => {
    if (!item.label) return 'inherit';

    return colors[item.label.color] ?? 'inherit';
  }, [colors, item]);

  return (
    <div
      className="character row p-y-5 p-b-5 m-t-2 relative text-1.2em"
      onMouseDown={() => updateCursor(item.index, 'down')}
      onMouseEnter={() => updateCursor(item.index, 'move')}
    >
      <span className="whitespace-pre selection:bg-[var(--highlight-color)]">{item.char}</span>
      {item.label && (
        <div style={{ backgroundColor: color }} className="absolute h-15px w-full bottom-0px" />
      )}
    </div>
  );
};

export default TextAnnotationCharacter;
