import axios from 'axios';
import cookies from 'js-cookie';

// const baseURL = process.env.API_URL;
const baseURL = 'http://localhost:8080';

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

export default $api;
