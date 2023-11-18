import { PropsWithChildren, createContext, useEffect, useMemo, useState } from 'react';
import { User } from '../types/user';

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
  const [user] = useState<User | undefined>(undefined);

  const isAuthenticated = useMemo(() => user !== undefined, [user]);

  const signIn: AuthData['signIn'] = async () => {};
  const signOut: AuthData['signOut'] = () => {};

  useEffect(() => {
    // TODO: one time effect that checks if the token and authenticate the user
    // TODO: add auth guard context that will redirect user to sign in page
  }, []);

  return (
    <AuthContext.Provider value={{ user, isAuthenticated, signIn, signOut }}>
      {children}
    </AuthContext.Provider>
  );
};
