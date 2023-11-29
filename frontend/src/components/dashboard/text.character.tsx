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
    if (!item.label) return 'transparent';

    return colors[item.label.color] ?? 'transparent';
  }, [colors, item]);

  return (
    <div
      className="character row whitespace p-y-5 p-b-5 m-t-2 relative"
      onMouseDown={() => updateCursor(item.index, 'down')}
      onMouseEnter={() => updateCursor(item.index, 'move')}
    >
      {item.char}
      {item.label && (
        <div style={{ backgroundColor: color }} className="absolute h-10px w-full bottom-0px" />
      )}
    </div>
  );
};

export default TextAnnotationCharacter;
