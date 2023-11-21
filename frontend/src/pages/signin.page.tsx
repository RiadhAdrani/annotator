import { Button, Input, Paper, Title, Text } from '@mantine/core';
import { useContext, useMemo, useState } from 'react';
import { Link } from 'react-router-dom';
import { SignInBody } from '../types/user';
import AuthContext from '../contexts/Auth.context';

interface FormField {
  key: keyof SignInBody;
  value: string;
  label: string;
  placeholder: string;
  error?: string;
  type?: string;
}

const SignInPage = () => {
  const { signIn } = useContext(AuthContext);

  const [body, setBody] = useState<SignInBody>({
    login: '',
    password: '',
  });

  const form = useMemo<Array<FormField>>(() => {
    return Object.keys(body).map((k) => {
      const key = k as keyof SignInBody;

      const out: FormField = { key, value: body[key], label: '', placeholder: '', type: 'text' };

      if (key === 'password') {
        out.label = 'Password';
        out.placeholder = 'Password';
        out.type = 'password';
      } else {
        out.label = 'Email or Username';
        out.placeholder = 'Email or username';
      }

      return out;
    });
  }, [body]);

  const onSubmit = async () => {
    const isError = form.some((f) => f.error !== undefined);

    if (isError) {
      return;
    }

    signIn(body.login, body.password);
  };

  return (
    <div className="flex-1 col m-y-auto">
      <Paper shadow="xs" p="xl" className="m-auto w-400px">
        <form className="col gap-5">
          <div>
            <Link to={'/'}>
              <Text>Annotator</Text>
            </Link>
            <Title>Sign in</Title>
          </div>
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
            <Link to="/sign-up">
              <Text size="sm">No account yet ?</Text>
            </Link>
            <Button onClick={onSubmit}>Sign in</Button>
          </div>
        </form>
      </Paper>
    </div>
  );
};

export default SignInPage;
