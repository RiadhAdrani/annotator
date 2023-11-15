'use client';

import $api from '@/helpers/api';
import { createContext, PropsWithChildren, useEffect, useState } from 'react';

export interface AppContextType {
  labelColors: Record<string, string>;
}

export const AppContext = createContext<AppContextType>({ labelColors: {} });

export const AppContextProvider = ({ children }: PropsWithChildren) => {
  const [labelColors, setLabelColors] = useState<Record<string, string>>({});

  // load data once
  useEffect(() => {
    $api
      .get('/data')
      .then((it) => {})
      .catch((e) => {
        console.log(e);
      });
  }, []);

  return <AppContext.Provider value={{ labelColors }}>{children}</AppContext.Provider>;
};
