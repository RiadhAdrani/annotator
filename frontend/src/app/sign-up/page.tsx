'use client';

import $api from '@/helpers/api';
import { SignUpBody } from '@/types/user';
import { Button, Container, Input, Text } from '@mantine/core';
import { FormEvent, useState } from 'react';

const SignIn = () => {
  // TODO: if user is authenticated, redirect

  const [form, setForm] = useState<SignUpBody>({
    email: '',
    firstname: '',
    lastname: '',
    password: '',
    username: '',
  });

  const update = (field: keyof SignUpBody, e: FormEvent<HTMLInputElement>) =>
    setForm({ ...form, [field]: e.currentTarget.value });

  const onSubmit = () => {
    $api.post('/echo', JSON.stringify(form)).then((it) => {
      console.log(it.data);
    });
  };

  return (
    <>
      <Container className="flex flex- gap-2">
        <Text>Sign up</Text>
        <Input placeholder="Email" onInput={(e) => update('email', e)} />
        <Input placeholder="Username" onInput={(e) => update('username', e)} />
        <Input placeholder="First Name" onInput={(e) => update('firstname', e)} />
        <Input placeholder="Last Name" onInput={(e) => update('lastname', e)} />
        <Input placeholder="Password" onInput={(e) => update('password', e)} />
        <Button onClick={onSubmit}>Create account</Button>
      </Container>
    </>
  );
};

export default SignIn;
