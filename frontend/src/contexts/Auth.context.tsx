import { PropsWithChildren, createContext, useCallback, useEffect, useMemo, useState } from 'react';
import { User, UserAuthResponse } from '../types/user';
import $api from '../utils/api';
import cookies from 'js-cookie';

export interface AuthData {
  user?: User;
  isAuthenticated: boolean;
  signIn: (login: string, passowrd: string) => Promise<void>;
  signOut: () => void;
}

const AuthContext = createContext<AuthData>({
  isAuthenticated: false,
  signIn: async () => undefined,
  signOut: () => undefined,
  user: undefined,
});

export default AuthContext;

export const AuthProvider = ({ children }: PropsWithChildren) => {
  const [user, setUser] = useState<User | undefined>(undefined);

  const isAuthenticated = useMemo(() => user !== undefined, [user]);

  const signInWithToken = useCallback(async () => {
    const token = cookies.get('token');

    if (!token) return;

    const res = await $api.get<User>('/users/me');

    setUser(res.data);
  }, []);

  const signIn: AuthData['signIn'] = async (login, password) => {
    const res = await $api.post<UserAuthResponse>('/auth/sign-in', { login, password });

    const token = res.data.token;

    cookies.set('token', token);

    signInWithToken();
  };

  const signOut: AuthData['signOut'] = () => {
    cookies.remove('token');

    setUser(undefined);
  };

  // ? will run on startup only
  useEffect(() => {
    signInWithToken();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <AuthContext.Provider value={{ user, isAuthenticated, signIn, signOut }}>
      {children}
    </AuthContext.Provider>
  );
};
