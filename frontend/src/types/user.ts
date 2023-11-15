export interface ObjectId {
  $oid: string;
}

export interface User {
  id: ObjectId;
  firstname: string;
  lastname: string;
  email: string;
  username: string;
  password: string;
}

export type UpdateUserBody = Partial<Pick<User, 'firstname' | 'lastname' | 'password'>>;

export type SignUpBody = Pick<User, 'firstname' | 'email' | 'password' | 'lastname' | 'username'>;

export interface SignInBody {
  login: string;
  password: string;
}
