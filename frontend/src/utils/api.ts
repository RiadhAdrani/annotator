import axios from 'axios';

import cookies from 'js-cookie';
import { toast } from 'sonner';

// const baseURL = process.env.API_URL;
const baseURL = import.meta.env.API_URL;

const $api = axios.create({
  baseURL,
  withCredentials: false,
});

$api.interceptors.request.use((config) => {
  const token = cookies.get('token');

  if (token && !config.headers.get('Authorization')) {
    config.headers.set('Authorization', `Bearer ${token}`);
  }

  return config;
});

$api.interceptors.response.use(
  (res) => res,
  (err) => {
    const msg: string | undefined = err.response?.data?.msg;

    toast.error(msg ?? 'Something went wrong...');
  }
);

export default $api;
