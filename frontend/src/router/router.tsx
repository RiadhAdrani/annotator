import { Outlet, RouterProvider, createBrowserRouter } from 'react-router-dom';
import { AuthProvider } from '../contexts/App.context';
import { MantineProvider, createTheme } from '@mantine/core';

import '../index.css';
import '@mantine/core/styles.css';
import 'uno.css';
import SignUpPage from '../pages/signup.page';
import SignInPage from '../pages/signin.page';
import HomePage from '../pages/home.page';
import { AuthGuardProvider } from '../contexts/AuthGuard.context';
import { Toaster } from 'sonner';
import DashboardPage from '../pages/dashboard/dashboard.page';
import { DashboardProvider } from '../contexts/Dashboard.context';
import DashboardFeaturedPage from '../pages/dashboard/dashboard.featured.page';
import TextAnnotationPage from '../pages/dashboard/dashboard.text.page';

const theme = createTheme({});

const Router = createBrowserRouter([
  {
    element: (
      <MantineProvider theme={theme}>
        <AuthProvider>
          <Toaster />
          <Outlet />
        </AuthProvider>
      </MantineProvider>
    ),
    children: [
      {
        path: '/',
        element: <HomePage />,
      },
      {
        element: (
          <AuthGuardProvider block="signed-out">
            <Outlet />
          </AuthGuardProvider>
        ),
        children: [
          {
            path: '/dashboard',
            element: (
              <DashboardProvider>
                <DashboardPage />
              </DashboardProvider>
            ),
            children: [
              { path: '/dashboard/featured', element: <DashboardFeaturedPage /> },
              { path: '/dashboard/text', element: <div>Text annotation</div> },
              { path: '/dashboard/text/:id', element: <TextAnnotationPage /> },
            ],
          },
        ],
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
