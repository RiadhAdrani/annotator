import { useContext, useMemo, useState } from 'react';
import { Character, TextAnnotationContext } from '../../contexts/TextAnnotation.context';
import AppContext from '../../contexts/App.context';
import { Menu, Tooltip } from '@mantine/core';

export interface TextCharacterProps {
  item: Character;
}

const TextAnnotationCharacter = ({ item }: TextCharacterProps) => {
  const { updateCursor, cursor, toggleCtxMenu, annotation, deleteToken } =
    useContext(TextAnnotationContext);
  const { colors } = useContext(AppContext);

  const [showMenu, setShowMenu] = useState(false);

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

  const ctxMenu = useMemo(() => {
    if (!item.label || !item.token) return [];

    const id = item.token._id.$oid;

    return [{ label: 'Delete', onClick: () => deleteToken(id) }];
  }, [item, deleteToken]);

  return (
    <div
      className="character row p-t-3 p-b-5 m-t-2 relative text-1.2em"
      onMouseDown={(e) => {
        if (e.button !== 0) return;

        updateCursor(item.index, 'down');
      }}
      onMouseEnter={() => updateCursor(item.index, 'move')}
    >
      <span
        className={`whitespace-pre selection:bg-[var(--highlight-color)] ${
          isHighlighted ? 'font-bold' : ''
        }`}
      >
        {item.char}
      </span>
      {item.label && (
        <Menu
          opened={!cursor.inProgress && showMenu}
          width={250}
          onClose={() => {
            toggleCtxMenu(false);
            setShowMenu(false);
          }}
        >
          <Menu.Target>
            <Tooltip label={item.label.name} withArrow>
              <div
                style={{ backgroundColor: color }}
                className="absolute h-15px w-full top-0px"
                onContextMenu={(e) => {
                  e.preventDefault();
                  e.stopPropagation();

                  toggleCtxMenu(true);
                  setShowMenu(true);
                }}
              />
            </Tooltip>
          </Menu.Target>
          <Menu.Dropdown>
            <Menu.Label>
              <div>Label : "{item.label.name}"</div>
            </Menu.Label>
            <Menu.Label>
              <div>
                Content : "
                {annotation?.content.substring(item.token?.start ?? 0, (item.token?.end ?? 0) + 1)}"
              </div>
            </Menu.Label>
            <Menu.Divider />
            {ctxMenu.map((it, key) => (
              <Menu.Item key={key}>
                <div onClick={it.onClick}>{it.label}</div>
              </Menu.Item>
            ))}
          </Menu.Dropdown>
        </Menu>
      )}
    </div>
  );
};

export default TextAnnotationCharacter;
