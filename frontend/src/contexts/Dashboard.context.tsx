import {
  PropsWithChildren,
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
} from 'react';
import { CreateTextAnnotationBody, TextAnnotation } from '../types/annotations';
import $api from '../utils/api';
import AppContext from './App.context';

export interface DashboardData {
  isNavBarOpened: boolean;
  toggleNavBarOpened: (v?: boolean) => void;
  textAnnotations: Array<TextAnnotation>;
  createTextAnnotation: (body: CreateTextAnnotationBody) => Promise<void>;
  deleteTextAnnotation: (id: string) => Promise<void>;
}

export const DashboardContext = createContext<DashboardData>({
  isNavBarOpened: true,
  textAnnotations: [],
  toggleNavBarOpened: () => 0,
  createTextAnnotation: async () => undefined,
  deleteTextAnnotation: async () => undefined,
});

export const DashboardProvider = ({ children }: PropsWithChildren) => {
  const { isAuthenticated } = useContext(AppContext);

  const [isNavBarOpened, setNavBarOpened] = useState(false);

  const [textAnnotations, setTextAnnotations] = useState<Array<TextAnnotation>>([]);

  const toggleNavBarOpened = useCallback(
    (v?: boolean) => {
      setNavBarOpened(typeof v === 'boolean' ? v : !isNavBarOpened);
    },
    [isNavBarOpened]
  );

  const createTextAnnotation = useCallback(async (body: CreateTextAnnotationBody) => {
    const res = await $api.post<TextAnnotation>('/annotations/text/', body);

    setTextAnnotations((v) => [...v, res.data]);
  }, []);

  const deleteTextAnnotation = useCallback(async (id: string) => {
    await $api.delete(`/annotations/text/${id}`);

    setTextAnnotations((v) => v.filter((it) => it._id.$oid !== id));
  }, []);

  const fetchTextAnnotations = useCallback(async (page = 1, count = 10) => {
    const res = await $api.get<Array<TextAnnotation>>(
      `/annotations/text/?page=${page}&count=${count}`
    );

    setTextAnnotations(res.data);
  }, []);

  useEffect(() => {
    if (!isAuthenticated) return;

    fetchTextAnnotations();

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <DashboardContext.Provider
      value={{
        isNavBarOpened,
        toggleNavBarOpened,
        textAnnotations,
        createTextAnnotation,
        deleteTextAnnotation,
      }}
    >
      {children}
    </DashboardContext.Provider>
  );
};
