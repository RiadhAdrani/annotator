import { Button, Paper, Text, Title } from '@mantine/core';
import { useContext } from 'react';
import { Link } from 'react-router-dom';
import AppContext from '../contexts/App.context';

const HomePage = () => {
  const { isAuthenticated, signOut } = useContext(AppContext);

  return (
    <div className="col-center gap-5 flex-1">
      <Paper shadow="lg" p={50} className="col-center gap-5">
        <div className="col-center">
          <Title>Annotator</Title>
          <Text>The best annotation tool</Text>
        </div>
        {isAuthenticated ? (
          <div className="row-center gap-3">
            <Button onClick={signOut}>Sign out</Button>
            <Link to={'/dashboard'}>
              <Button>Dashboard</Button>
            </Link>
          </div>
        ) : (
          <div className="row-center gap-3">
            <Link to={'/sign-in'}>
              <Button>Sign in</Button>
            </Link>
            <Link to={'/sign-up'}>
              <Button>Sign up</Button>
            </Link>
          </div>
        )}
      </Paper>
    </div>
  );
};

export default HomePage;
