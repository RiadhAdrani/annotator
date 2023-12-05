import { useContext, useMemo } from 'react';
import AppContext from '../../contexts/App.context';
import { Button, Tooltip } from '@mantine/core';

export interface LabelColorPickerProps {
  selected?: string;
  filter: Array<string>;
  onSelected: (color: string) => void;
}

const LabelColorPicker = ({ filter, onSelected, selected }: LabelColorPickerProps) => {
  const { colors } = useContext(AppContext);

  const available = useMemo(() => {
    return Object.keys(colors)
      .filter((color) => !filter.includes(color))
      .map((it) => ({ name: it, color: colors[it], selected: it === selected }));
  }, [colors, filter, selected]);

  return (
    <div className="row flex-wrap gap-2">
      {available.map((color) => (
        <>
          <Tooltip label={color.name}>
            <Button style={{ backgroundColor: color.color }} onClick={() => onSelected(color.name)}>
              {color.selected && <i className="i-mdi-check-circle" />}
            </Button>
          </Tooltip>
        </>
      ))}
    </div>
  );
};

export default LabelColorPicker;
