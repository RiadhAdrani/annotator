import { Outlet, RouterProvider, createBrowserRouter } from 'react-router-dom';
import { AuthProvider } from '../contexts/Auth.context';
import { MantineProvider, Title, createTheme } from '@mantine/core';

import '../index.css';
import '@mantine/core/styles.css';
import 'uno.css';
import SignUpPage from '../pages/signup.page';

const theme = createTheme({});

const Router = createBrowserRouter([
  {
    element: (
      <AuthProvider>
        <MantineProvider theme={theme}>
          {/* // TODO: navbar */}
          <div className="col min-h-100vh">
            <nav className="bg-zinc-300 py-1">
              <Title>Annotator</Title>
            </nav>
            <Outlet />
          </div>
        </MantineProvider>
      </AuthProvider>
    ),
    children: [
      // TODO: protect with auth
      {
        element: <Outlet />,
      },
      // available globally
      {
        path: '/',
        element: <div>Home</div>,
      },
      {
        path: '/sign-up',
        element: <SignUpPage />,
      },
    ],
  },
]);

const App = () => {
  return <RouterProvider router={Router} />;
};

export default App;
