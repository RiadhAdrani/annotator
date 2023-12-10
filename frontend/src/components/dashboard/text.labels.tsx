import { useCallback, useContext, useMemo, useState } from 'react';
import { TextAnnotationContext } from '../../contexts/TextAnnotation.context';
import AppContext from '../../contexts/App.context';
import {
  Button,
  Chip,
  LoadingOverlay,
  Menu,
  MenuDivider,
  MenuDropdown,
  MenuItem,
  MenuTarget,
  Modal,
  Skeleton,
  Text,
  TextInput,
} from '@mantine/core';
import { Label } from '../../types/annotations';
import { generateContrastSafeColor, changeColorOpacity } from '@riadh-adrani/color-utils';
import CreateLabelModal from './createLabel.modal';
import LabelColorPicker from './labelColorPicker';
import ConfirmModal from '../modal/confirm.modal';

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
          <Button onClick={() => setOpened(true)} className="m-r-5">
            Create label
          </Button>
          <CreateLabelModal opened={opened} onClose={() => setOpened(false)} />
          {annotation.labels.length === 0 ? (
            <div className="text-gray">To begin annotating, start by creating a new label</div>
          ) : (
            annotation.labels.map((label) => <LabelChip key={label._id.$oid} label={label} />)
          )}
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

  const { selectedLabel, selectLabel, deleteLabel, annotation } = useContext(TextAnnotationContext);

  const [showUpdateModal, setShowUpdateModal] = useState(false);
  const [showDeleteModal, setShowDeleteModal] = useState(false);

  const isSelected = useMemo(() => selectedLabel === label._id.$oid, [label, selectedLabel]);

  const color = useMemo(() => colors[label.color], [label, colors]);

  const useCount = useMemo(
    () => annotation?.tokens.filter((it) => it.label?.$oid === label._id.$oid).length ?? 0,
    [annotation?.tokens, label]
  );

  return (
    <>
      <Chip
        checked={isSelected}
        style={{
          '--chip-bg': color,
          '--chip-hover': changeColorOpacity(color, 0.75),
          '--chip-color': generateContrastSafeColor(color),
          '--chip-icon': generateContrastSafeColor(color),
        }}
        className="group label-chip"
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
              <MenuItem disabled={isSelected} onClick={() => selectLabel(label._id.$oid)}>
                Select
              </MenuItem>
              <MenuDivider />
              <MenuItem onClick={() => setShowUpdateModal(true)}>Update</MenuItem>
              <MenuItem onClick={() => setShowDeleteModal(true)}>Delete</MenuItem>
            </MenuDropdown>
          </Menu>
        </div>
      </Chip>
      <Modal opened={showUpdateModal} centered onClose={() => setShowUpdateModal(false)}>
        <Modal.Body>
          <UpdateLabelModal label={label} close={() => setShowUpdateModal(false)} />
        </Modal.Body>
      </Modal>
      <ConfirmModal
        title="Are you sure you want to delete this label"
        opened={showDeleteModal}
        onClose={() => setShowDeleteModal(false)}
        onAccept={() => {
          deleteLabel(label._id.$oid);
        }}
      >
        {useCount > 0 && <span className="text-red">This label is used by some tokens !</span>}
      </ConfirmModal>
    </>
  );
};

const UpdateLabelModal = ({ label, close }: { label: Label; close: () => void }) => {
  const { annotation, updateLabel } = useContext(TextAnnotationContext);

  const [isLoading, setLoading] = useState(false);

  const [name, setName] = useState(label.name);
  const [color, setColor] = useState(label.color);

  const usedColors = useMemo(() => {
    if (!annotation) return [];

    return annotation.labels.map((it) => (it._id.$oid === label._id.$oid ? '' : it.color));
  }, [annotation, label]);

  const canUpdate = useMemo(
    () => (name !== label.name || color !== label.color) && name.trim(),
    [name, color, label]
  );

  const update = useCallback(async () => {
    if (!canUpdate) return;

    const body: Partial<Pick<Label, 'color' | 'name'>> = {};

    if (name !== label.name) {
      body.name = name;
    }
    if (color !== label.color) {
      body.color = color;
    }

    setLoading(true);

    updateLabel(label._id.$oid, body)
      .then(() => {
        close();
      })
      .finally(() => {
        setLoading(false);
      });
  }, [name, color, canUpdate, label, updateLabel, close]);

  return (
    <>
      <LoadingOverlay visible={isLoading} />
      <div className="col gap-8">
        <div>
          <Text c={'gray'} size="xs">
            {label._id.$oid}
          </Text>
          <div>Update Label</div>
        </div>
        <div className="col gap-3">
          <TextInput
            value={name}
            placeholder="Name"
            onInput={(e) => setName(e.currentTarget.value)}
          />
          <LabelColorPicker filter={usedColors} selected={color} onSelected={(c) => setColor(c)} />
        </div>
        <div className="row justify-end gap-3">
          <Button onClick={close} variant="light">
            Close
          </Button>
          <Button disabled={!canUpdate} onClick={update}>
            Save
          </Button>
        </div>
      </div>
    </>
  );
};

export default TextAnnotationLabels;
