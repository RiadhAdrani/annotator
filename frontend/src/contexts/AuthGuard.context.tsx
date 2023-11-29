import { PropsWithChildren, useContext, useEffect, useMemo } from 'react';
import AppContext from './App.context';
import { useNavigate } from 'react-router-dom';

export const AuthGuardProvider = ({
  children,
  block,
}: PropsWithChildren<{ block: 'signed-out' | 'signed-in' }>) => {
  const { isAuthenticated } = useContext(AppContext);

  const navigate = useNavigate();

  const isBlocked = useMemo(() => {
    if (isAuthenticated && block === 'signed-in') {
      return true;
    }

    if (!isAuthenticated && block === 'signed-out') {
      return true;
    }

    return false;
  }, [block, isAuthenticated]);

  useEffect(() => {
    if (!isBlocked) return;

    navigate(block === 'signed-in' ? '/' : '/sign-in');
  }, [isBlocked, navigate, block]);

  return <>{children}</>;
};
