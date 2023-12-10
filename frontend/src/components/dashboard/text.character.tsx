import { useContext, useMemo } from 'react';
import { Character, TextAnnotationContext } from '../../contexts/TextAnnotation.context';
import AppContext from '../../contexts/App.context';

export interface TextCharacterProps {
  item: Character;
}

const TextAnnotationCharacter = ({ item }: TextCharacterProps) => {
  const { updateCursor, cursor, finishCursor } = useContext(TextAnnotationContext);
  const { colors } = useContext(AppContext);

  const color = useMemo(() => {
    if (!item.label) return 'inherit';

    return colors[item.label.color] ?? 'inherit';
  }, [colors, item]);

  const isHighlighted = useMemo(() => {
    const { inProgress, end, start } = cursor;

    if (!inProgress) return false;

    const min = Math.min(end, start);
    const max = Math.max(end, start);

    return min <= item.index && item.index <= max;
  }, [cursor, item.index]);

  return (
    <div
      className="character row p-t-3 p-b-5 m-t-2 relative text-1.2em"
      onMouseDown={(e) => {
        if (e.button !== 0) return;

        updateCursor(item.index, 'down');
      }}
      onMouseEnter={() => updateCursor(item.index, 'move')}
      onMouseUp={finishCursor}
    >
      <span
        className={`whitespace-pre selection:bg-[var(--highlight-color)] ${
          isHighlighted ? 'font-bold' : ''
        }`}
      >
        {item.char}
      </span>
      {item.label && (
        <div style={{ backgroundColor: color }} className="absolute h-8px w-full top-0px" />
      )}
    </div>
  );
};

export default TextAnnotationCharacter;
