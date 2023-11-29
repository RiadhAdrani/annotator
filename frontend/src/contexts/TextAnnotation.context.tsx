import { PropsWithChildren, createContext, useCallback, useEffect, useMemo, useState } from 'react';
import { TextAnnotation } from '../types/annotations';
import $api from '../utils/api';
import { toast } from 'sonner';
import { useParams } from 'react-router-dom';

export interface Character {
  char: string;
  index: number;
  label?: { name: string; color: string };
}

export interface Word {
  characters: Array<Character>;
}

export interface Paragraph {
  words: Array<Word>;
}

export interface TextAnnotationContextData {
  annotation?: TextAnnotation;
  paragraphs: Array<Paragraph>;
  selectedLabel: string | undefined;

  createLabel: (body: { name: string; color: string }) => Promise<void>;
  selectLabel: (id: string) => void;
  deleteLabel: (id: string) => Promise<void>;
  updateCursor: (index: number, event: 'down' | 'move') => void;
}

export const TextAnnotationContext = createContext<TextAnnotationContextData>({
  paragraphs: [],
  selectedLabel: undefined,

  createLabel: async () => undefined,
  deleteLabel: async () => undefined,
  selectLabel: () => undefined,
  updateCursor: () => undefined,
});

export const TextAnnotationProvider = ({ children }: PropsWithChildren) => {
  const { id } = useParams();

  const [annotation, setAnnotation] = useState<TextAnnotation | undefined>();

  const [, setFailed] = useState(false);
  const [cursor, setCursor] = useState({ start: -1, end: -1, inProgress: false });

  const [selectedLabel, setSelectedLabel] = useState<string | undefined>();

  const paragraphs = useMemo<Array<Paragraph>>(() => {
    if (!annotation) return [];

    let paragraph: Paragraph | undefined;
    let word: Word | undefined;

    return annotation.content.split('').reduce((acc, char, index) => {
      paragraph ??= { words: [] };

      if (char === '\n') {
        // finish current paragraph
        acc.push(paragraph);

        paragraph = undefined;
      } else {
        // make sure that word is defined
        word ??= { characters: [] };

        // check if tokenized
        const token = annotation.tokens.find((t) => t.start <= index && index <= t.end);

        let label: Character['label'] | undefined;

        if (token?.label?.$oid) {
          const l = annotation.labels.find((it) => it._id.$oid === token.label?.$oid);

          if (l) {
            label = { color: l.color, name: l.name };
          } else {
            toast.error(`Unable to find label with id ${token.label.$oid}`);
          }
        }

        word.characters.push({ char, index, label });

        if (char === ' ') {
          // finish current word
          paragraph.words.push(word);

          word = undefined;
        }

        // reached the end
        if (index === annotation.content.length - 1) {
          paragraph.words.push(word as Word);

          acc.push(paragraph);
        }
      }

      return acc;
    }, [] as Array<Paragraph>);
  }, [annotation]);

  const createLabel = useCallback(
    async (body: { name: string; color: string }) => {
      if (!annotation || !id) return;

      return $api
        .post<TextAnnotation>(`/annotations/text/${id}/labels`, body)
        .then((it) => setAnnotation(it.data));
    },
    [annotation, id]
  );

  const deleteLabel = useCallback(
    async (labelId: string) => {
      if (!annotation || !id) return;

      return $api
        .delete<TextAnnotation>(`/annotations/text/${id}/labels/${labelId}`)
        .then((it) => setAnnotation(it.data));
    },
    [annotation, id]
  );

  const selectLabel = useCallback(
    (labelId: string) => {
      if (!annotation || !id || labelId === selectedLabel) return;

      const exists = annotation.labels.some((it) => it._id.$oid === labelId);

      if (!exists) {
        toast.error('Label does not exist');
      }

      setSelectedLabel(labelId);
    },
    [annotation, id, selectedLabel]
  );

  const updateCursor = useCallback(
    (index: number, event: 'down' | 'move') => {
      if (!annotation || !id || !selectedLabel) return;

      if (event === 'down') {
        setCursor({ end: index, inProgress: true, start: index });
      } else if (event === 'move') {
        setCursor((c) => ({ ...c, end: index }));
      }
    },
    [annotation, id, selectedLabel]
  );

  useEffect(() => {
    if (!id) return;

    $api
      .get<TextAnnotation>(`/annotations/text/${id}`)
      .then((it) => {
        setTimeout(() => {
          setAnnotation(it.data);

          // set selected label if exist
          if (it.data.labels[0]) {
            setSelectedLabel(it.data.labels[0]._id.$oid);
          }
        }, 1000);
      })
      .catch(() => {
        setFailed(true);
      });
  }, [id]);

  useEffect(() => {
    if (!annotation) return;

    const listener: (e: MouseEvent) => void = () => {
      // should have valid state
      setCursor({ end: -1, start: -1, inProgress: false });

      if (!cursor.inProgress) {
        return;
      }

      if (!selectedLabel) {
        return toast.error('Please select a label before annotating...');
      }

      const start = Math.min(cursor.start, cursor.end);
      const end = Math.max(cursor.start, cursor.end);

      const body = { start, end, label: selectedLabel };

      console.log(body);

      $api
        .post<TextAnnotation>(`/annotations/text/${annotation?._id.$oid}/tokens`, body)
        .then((it) => setAnnotation(it.data));
    };

    window.addEventListener('mouseup', listener);

    return () => window.removeEventListener('mouseup', listener);
  }, [cursor, selectedLabel, annotation]);

  return (
    <TextAnnotationContext.Provider
      value={{
        paragraphs,
        annotation,
        updateCursor,
        selectLabel,
        deleteLabel,
        createLabel,
        selectedLabel,
      }}
    >
      {children}
    </TextAnnotationContext.Provider>
  );
};
