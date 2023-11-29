import { Base } from './base';

export interface User extends Base {
  firstname: string;
  lastname: string;
  email: string;
  username: string;
}

export interface CreateUserBody extends Omit<User, '_id'> {
  password: string;
}

export type UpdateUserBody = Partial<Pick<User, 'firstname' | 'lastname' | 'username'>>;

export interface SignInBody {
  login: string;
  password: string;
}

export interface UserAuthResponse {
  token: string;
}
