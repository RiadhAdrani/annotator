import { Button, Input, Paper, Title, Text } from '@mantine/core';
import { useMemo, useState } from 'react';
import { Link } from 'react-router-dom';
import { CreateUserBody } from '../types/user';
import $api from '../utils/api';

interface FormField {
  key: keyof CreateUserBody;
  value: string;
  label: string;
  placeholder: string;
  error?: string;
  type?: string;
}

const SignInPage = () => {
  const [body, setBody] = useState<CreateUserBody>({
    username: '',
    email: '',
    firstname: '',
    lastname: '',
    password: '',
  });

  const form = useMemo<Array<FormField>>(() => {
    return Object.keys(body).map((k) => {
      const key = k as keyof CreateUserBody;

      const out: FormField = { key, value: body[key], label: '', placeholder: '', type: 'text' };

      if (key === 'firstname') {
        out.label = 'First Name';
        out.placeholder = 'First Name';
      } else if (key === 'email') {
        out.label = 'Email';
        out.placeholder = 'Email';
        out.type = 'email';
      } else if (key === 'password') {
        out.label = 'Password';
        out.placeholder = 'Password';
        out.type = 'password';
      } else if (key === 'lastname') {
        out.label = 'Last Name';
        out.placeholder = 'Last Name';
      } else if (key === 'username') {
        out.label = 'Username';
        out.placeholder = 'Username';
      }

      return out;
    });
  }, [body]);

  const onSubmit = async () => {
    const isError = form.some((f) => f.error !== undefined);

    if (isError) {
      return;
    }

    $api.post('/auth/sign-up', body);

    // TODO: register token and redirect user
  };

  return (
    <div className="flex-1 col m-y-auto">
      <Paper shadow="xs" p="xl" className="m-auto w-400px">
        <form className="col gap-5">
          <Title>Sign in</Title>
          <div className="col gap-2">
            {form.map((item) => (
              <Input.Wrapper key={item.key} label={item.label} error={item.error}>
                <Input
                  type={item.type}
                  placeholder={item.placeholder}
                  value={item.value}
                  onInput={(e) => setBody({ ...body, [item.key]: e.currentTarget.value })}
                />
              </Input.Wrapper>
            ))}
          </div>
          <div className="row-center justify-between">
            <Link to="/sign-in">
              <Text size="sm">Already have an account</Text>
            </Link>
            <Button onClick={onSubmit}>Create account</Button>
          </div>
        </form>
      </Paper>
    </div>
  );
};

export default SignInPage;
