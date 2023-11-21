import { Outlet, RouterProvider, createBrowserRouter } from 'react-router-dom';
import { AuthProvider } from '../contexts/Auth.context';
import { MantineProvider, createTheme } from '@mantine/core';

import '../index.css';
import '@mantine/core/styles.css';
import 'uno.css';
import SignUpPage from '../pages/signup.page';
import SignInPage from '../pages/signin.page';
import HomePage from '../pages/home.page';
import { AuthGuardProvider } from '../contexts/AuthGuard.context';

const theme = createTheme({});

const Router = createBrowserRouter([
  {
    element: (
      <AuthProvider>
        <MantineProvider theme={theme}>
          <Outlet />
        </MantineProvider>
      </AuthProvider>
    ),
    children: [
      {
        path: '/',
        element: <HomePage />,
      },
      {
        element: (
          <AuthGuardProvider block="signed-in">
            <Outlet />
          </AuthGuardProvider>
        ),
        children: [
          {
            path: '/sign-up',
            element: <SignUpPage />,
          },
          {
            path: '/sign-in',
            element: <SignInPage />,
          },
        ],
      },
    ],
  },
]);

const App = () => {
  return <RouterProvider router={Router} />;
};

export default App;
