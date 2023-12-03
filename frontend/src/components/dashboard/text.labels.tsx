import { useContext, useMemo, useState } from 'react';
import { TextAnnotationContext } from '../../contexts/TextAnnotation.context';
import AppContext from '../../contexts/App.context';
import {
  Button,
  Chip,
  Menu,
  MenuDivider,
  MenuDropdown,
  MenuItem,
  MenuTarget,
  Skeleton,
} from '@mantine/core';
import { Label } from '../../types/annotations';
import { generateContrastSafeColor, changeColorOpacity } from '@riadh-adrani/color-utils';
import CreateLabelModal from './createLabel.modal';

const TextAnnotationLabels = () => {
  const { annotation } = useContext(TextAnnotationContext);

  const [opened, setOpened] = useState(false);

  return (
    <div className="row flex-wrap gap-2 mb-5 items-center">
      {!annotation ? (
        <>
          <Skeleton height={35} width={100} />
          {new Array(10).fill(0).map((v, i) => (
            <Skeleton key={`${v}-${i}`} height={30} width={75} radius={20} />
          ))}
        </>
      ) : (
        <>
          <Button onClick={() => setOpened(true)}>Create label</Button>
          <CreateLabelModal opened={opened} onClose={() => setOpened(false)} />
          {annotation.labels.map((label) => (
            <LabelChip key={label._id.$oid} label={label} />
          ))}
        </>
      )}
    </div>
  );
};

interface LabelChipProps {
  label: Label;
}

const LabelChip = ({ label }: LabelChipProps) => {
  const { colors } = useContext(AppContext);

  const { selectedLabel, selectLabel, deleteLabel } = useContext(TextAnnotationContext);

  const isSelected = useMemo(() => selectedLabel === label._id.$oid, [label, selectedLabel]);

  const color = useMemo(() => colors[label.color], [label, colors]);

  return (
    <Chip
      checked={isSelected}
      style={{
        '--chip-bg': color,
        '--chip-hover': changeColorOpacity(color, 0.75),
        '--chip-color': generateContrastSafeColor(color),
        '--chip-icon': generateContrastSafeColor(color),
      }}
      className="group"
      onClick={() => selectLabel(label._id.$oid)}
    >
      <div className="row-center ml-3 gap-2">
        <span>{label.name}</span>
        <Menu>
          <MenuTarget>
            <Button
              size="compact-sm"
              variant="subtle"
              className="text-inherit opacity-0 group-hover:opacity-100"
            >
              <i className="i-mdi-more-vert" />
            </Button>
          </MenuTarget>
          <MenuDropdown>
            <MenuItem onClick={() => deleteLabel(label._id.$oid)}>Delete</MenuItem>
            <MenuDivider />
            <MenuItem onClick={() => selectLabel(label._id.$oid)}>Select</MenuItem>
          </MenuDropdown>
        </Menu>
      </div>
    </Chip>
  );
};

export default TextAnnotationLabels;
