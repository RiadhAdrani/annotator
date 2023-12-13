import { PropsWithChildren, createContext, useCallback, useEffect, useMemo, useState } from 'react';
import { TextAnnotation, Token } from '../types/annotations';
import $api from '../utils/api';
import { toast } from 'sonner';
import { useParams } from 'react-router-dom';

export interface Character {
  char: string;
  index: number;
  label?: { name: string; color: string };
  token?: Token;
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
  cursorHint: string;
  isHighlighting: boolean;

  createLabel: (body: { name: string; color: string }) => Promise<void>;
  deleteLabel: (id: string) => Promise<void>;
  updateLabel: (id: string, body: { name?: string; color?: string }) => Promise<void>;
  selectLabel: (id: string) => void;
  updateCursor: (index: number, event: 'down' | 'move') => void;
  cancelCursor: () => void;
  finishCursor: () => void;

  updateAnnotation: (body: { title: string }) => Promise<void>;

  deleteToken: (id: string) => void;

  toggleCtxMenu: (v?: boolean) => void;

  cursor: { inProgress: boolean; start: number; end: number };
}

export const TextAnnotationContext = createContext<TextAnnotationContextData>({
  paragraphs: [],
  selectedLabel: undefined,
  cursorHint: '',

  createLabel: async () => undefined,
  deleteLabel: async () => undefined,
  updateLabel: async () => undefined,
  selectLabel: () => undefined,
  updateCursor: () => undefined,
  cancelCursor: () => undefined,
  finishCursor: () => undefined,
  updateAnnotation: async () => undefined,
  isHighlighting: false,
  cursor: { end: -1, inProgress: false, start: -1 },
  toggleCtxMenu: () => 0,
  deleteToken: () => 0,
});

export const TextAnnotationProvider = ({ children }: PropsWithChildren) => {
  const { id } = useParams();

  const [annotation, setAnnotation] = useState<TextAnnotation | undefined>();

  const [, setFailed] = useState(false);
  const [cursor, setCursor] = useState({ start: -1, end: -1, inProgress: false });
  const [selectedLabel, setSelectedLabel] = useState<string | undefined>();

  const [ctxMenuShowed, setCtxMenuShowed] = useState(false);

  const canAnnotate = useMemo(() => {
    return !ctxMenuShowed;
  }, [ctxMenuShowed]);

  const cursorHint = useMemo(() => {
    if (!annotation) return '';

    if (!cursor.inProgress)
      return 'To start annotating, click and hold the mouse on a character, and then drag it to highlight the area you want to include';

    return 'Waiting for you to finish highlighting... Press "Escape" to cancel or move the move outside of the text box.';
  }, [annotation, cursor]);

  const isHighlighting = useMemo(() => {
    if (!annotation) return false;

    return cursor.inProgress;
  }, [annotation, cursor]);

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

        word.characters.push({ char, index, label, token });

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

  const toggleCtxMenu = useCallback((v?: boolean) => {
    setCtxMenuShowed((current) => (typeof v === 'boolean' ? v : !current));
  }, []);

  const createLabel = useCallback(
    async (body: { name: string; color: string }) => {
      if (!annotation || !id) return;

      return $api
        .post<TextAnnotation>(`/annotations/text/${id}/labels`, body)
        .then((it) => setAnnotation(it.data));
    },
    [annotation, id]
  );

  const updateLabel: TextAnnotationContextData['updateLabel'] = useCallback(
    async (labelId, body) => {
      if (!annotation || !id) return;

      return $api
        .put<TextAnnotation>(`/annotations/text/${id}/labels/${labelId}`, body)
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
      if (!annotation || !id || !selectedLabel || !canAnnotate) return;

      if (event === 'down') {
        setCursor({ end: index, inProgress: true, start: index });
      } else if (event === 'move') {
        setCursor((c) => ({ ...c, end: index }));
      }
    },
    [annotation, id, selectedLabel, canAnnotate]
  );

  const cancelCursor = useCallback(() => {
    if (!annotation || !cursor.inProgress) return;

    toast.warning('Annotation canceled');

    setCursor({ inProgress: false, end: -1, start: -1 });
  }, [annotation, cursor]);

  const finishCursor = useCallback(() => {
    if (!annotation || !cursor.inProgress) return;

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

    $api
      .post<TextAnnotation>(`/annotations/text/${annotation?._id.$oid}/tokens`, body)
      .then((it) => setAnnotation(it.data));
  }, [annotation, cursor, selectedLabel]);

  const deleteToken = useCallback(
    (id: string) => {
      if (!annotation) return;

      $api
        .delete<TextAnnotation>(`/annotations/text/${annotation._id.$oid}/tokens/${id}`)
        .then((it) => {
          setAnnotation(it.data);
          toast.info('Token deleted successfully');
        });
    },
    [annotation]
  );

  const updateAnnotation: TextAnnotationContextData['updateAnnotation'] = useCallback(
    async (body) => {
      if (!annotation) return;

      $api.put<TextAnnotation>(`/annotations/text/${annotation._id.$oid}`, body).then((it) => {
        setAnnotation(it.data);
        toast.info('Annotation updated successfully');
      });
    },
    [annotation]
  );

  useEffect(() => {
    if (!id) return;

    setAnnotation(undefined);

    $api
      .get<TextAnnotation>(`/annotations/text/${id}`)
      .then((it) => {
        setTimeout(() => {
          setAnnotation(it.data);

          // set selected label if exist
          if (it.data.labels[0]) {
            setSelectedLabel(it.data.labels[0]._id.$oid);
          }
        }, 200);
      })
      .catch(() => {
        setFailed(true);
      });
  }, [id]);

  useEffect(() => {
    if (!annotation) return;

    const escaped = (e: KeyboardEvent) => {
      const { key } = e;

      if (cursor.inProgress && key === 'Escape') {
        cancelCursor();
      }
    };

    window.addEventListener('keyup', escaped);

    return () => {
      window.removeEventListener('keyup', escaped);
    };
  }, [cursor, selectedLabel, annotation, cancelCursor]);

  return (
    <TextAnnotationContext.Provider
      value={{
        cursor,
        paragraphs,
        annotation,
        updateCursor,
        selectLabel,
        deleteLabel,
        createLabel,
        finishCursor,
        isHighlighting,
        cursorHint,
        selectedLabel,
        cancelCursor,
        updateLabel,
        updateAnnotation,
        deleteToken,
        toggleCtxMenu,
      }}
    >
      {children}
    </TextAnnotationContext.Provider>
  );
};
