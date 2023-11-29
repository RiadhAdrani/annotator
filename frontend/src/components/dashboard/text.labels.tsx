import { useContext } from 'react';
import { TextAnnotationContext } from '../../contexts/TextAnnotation.context';
import AppContext from '../../contexts/App.context';
import { Skeleton } from '@mantine/core';

const TextAnnotationLabels = () => {
  const { colors } = useContext(AppContext);
  const { annotation, selectedLabel, selectLabel } = useContext(TextAnnotationContext);

  return (
    <div className="row flex-wrap gap-2 mb-5">
      {!annotation ? (
        <>
          {new Array(10).map(() => (
            <Skeleton height={'50'} width={100} />
          ))}
        </>
      ) : (
        annotation.labels.map((label) => (
          <button
            key={label._id.$oid}
            className={`p-x-4 p-y-1 rounded-lg cursor-pointer border-none`}
            onClick={() => selectLabel(label._id.$oid)}
            style={{
              backgroundColor: colors[label.color],
              opacity: selectedLabel === label._id.$oid ? '1' : '0.5',
            }}
          >
            <span
              style={{
                background: colors[label.color],
                WebkitBackgroundClip: 'text',
                backgroundClip: 'text',
                color: 'transparent',
                filter: 'invert(1) grayscale(1) contrast(9)',
              }}
            >
              {label.name}
            </span>
          </button>
        ))
      )}
    </div>
  );
};

export default TextAnnotationLabels;
