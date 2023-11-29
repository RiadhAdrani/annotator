import { PropsWithChildren, createContext, useCallback, useEffect, useMemo, useState } from 'react';
import { CreateUserBody, User, UserAuthResponse } from '../types/user';
import $api from '../utils/api';
import cookies from 'js-cookie';
import { toast } from 'sonner';
import { Box, LoadingOverlay } from '@mantine/core';
import { useNavigate } from 'react-router-dom';

export interface AppData {
  user?: User;
  isAuthenticated: boolean;
  signIn: (login: string, passowrd: string) => Promise<void>;
  signOut: () => void;
  signUp: (body: CreateUserBody) => Promise<void>;
  colors: Record<string, string>;
}

const AppContext = createContext<AppData>({
  isAuthenticated: false,
  signIn: async () => undefined,
  signOut: () => undefined,
  signUp: async () => undefined,
  user: undefined,
  colors: {},
});

export default AppContext;

export const AuthProvider = ({ children }: PropsWithChildren) => {
  const navigate = useNavigate();

  const [user, setUser] = useState<User | undefined>(undefined);

  const isAuthenticated = useMemo(() => user !== undefined, [user]);

  const [colors, setColors] = useState<Record<string, string>>({});

  const [firstTime, setFirstTime] = useState(true);

  const signInWithToken = useCallback(async () => {
    const token = cookies.get('token');

    if (!token) {
      toast.error(`Token is invalid or missing, Sign in to use Annotator at its fullest.`);

      return;
    }

    try {
      const res = await $api.get<User>('/users/me');

      setUser(res.data);

      toast.info(`Signed in successfully, welcome "${res.data.firstname} ${res.data.lastname}"`);
    } catch (e) {
      cookies.remove('token');

      navigate('/sign-in');

      toast.error(`Token is invalid, please sign in again.`);
    }
  }, [navigate]);

  const signIn: AppData['signIn'] = async (login, password) => {
    const res = await $api.post<UserAuthResponse>('/auth/sign-in', { login, password });

    const token = res.data.token;

    cookies.set('token', token);

    signInWithToken();
  };

  const signOut: AppData['signOut'] = () => {
    cookies.remove('token');

    setUser(undefined);
  };

  const signUp: AppData['signUp'] = async (body) => {
    const res = await $api.post<UserAuthResponse>('/auth/sign-up', body);

    const token = res.data.token;

    cookies.set('token', token);

    signInWithToken();
  };

  // ? will run on startup only
  useEffect(() => {
    $api.get<{ colors: Record<string, string> }>('/app-data/').then((it) => {
      setColors(it.data.colors);
    });

    setTimeout(() => {
      signInWithToken().then(() => {
        setFirstTime(false);
      });
    }, 1000);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <AppContext.Provider value={{ user, isAuthenticated, signIn, signOut, signUp, colors }}>
      {firstTime ? (
        <Box pos={'fixed'} className="w-100vw h-100vw z-1000">
          <LoadingOverlay visible></LoadingOverlay>
        </Box>
      ) : (
        children
      )}
    </AppContext.Provider>
  );
};
