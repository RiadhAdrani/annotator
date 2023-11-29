import { AppShell, Avatar, Burger, Button, Menu, Text } from '@mantine/core';
import { useContext, useEffect } from 'react';
import { DashboardContext } from '../../contexts/Dashboard.context';
import { Link, Outlet, useLocation, useNavigate } from 'react-router-dom';
import AppContext from '../../contexts/App.context';

const DashboardPage = () => {
  const location = useLocation();
  const navigate = useNavigate();

  const { isNavBarOpened, toggleNavBarOpened } = useContext(DashboardContext);
  const { user, signOut } = useContext(AppContext);

  useEffect(() => {
    if (location.pathname === '/dashboard' || location.pathname === '/dashboard/') {
      navigate('/dashboard/featured');
    }
  }, [navigate, location.pathname]);

  return (
    <AppShell
      header={{ height: 60 }}
      navbar={{ width: 300, breakpoint: 'md', collapsed: { mobile: !isNavBarOpened } }}
    >
      <AppShell.Header className="row items-center pl-3">
        <div className="row-center gap-3">
          <Burger onClick={() => toggleNavBarOpened()} />
          <Link to={'/'}>
            <Text size="xl">Annotator</Text>
          </Link>
        </div>
        <div className="m-l-auto m-r-10">
          <Menu shadow="md" width={200}>
            <Menu.Target>
              <Button variant="transparent">
                <Avatar>
                  {user?.firstname[0]}
                  {user?.lastname[0]}
                </Avatar>
              </Button>
            </Menu.Target>
            <Menu.Dropdown>
              <Menu.Label>
                {user?.firstname} {user?.lastname}
              </Menu.Label>
              <Menu.Item>My account</Menu.Item>
              <Menu.Label>Actions</Menu.Label>
              <Menu.Item onClick={signOut}>Sign out</Menu.Item>
            </Menu.Dropdown>
          </Menu>
        </div>
      </AppShell.Header>
      <AppShell.Navbar p="md"></AppShell.Navbar>
      <AppShell.Main className="col">
        <div className="col p-5">
          <Outlet />
        </div>
      </AppShell.Main>
    </AppShell>
  );
};

export default DashboardPage;
