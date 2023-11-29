import { Base, ObjectId } from './base';

export interface Annotation extends Base {
  title: string;
  user_id: ObjectId;
}

export interface Label extends Base {
  name: string;
  color: string;
}

export interface Token extends Base {
  start: number;
  end: number;
  label: ObjectId | undefined;
}

export interface TextAnnotation extends Annotation {
  content: string;
  labels: Array<Label>;
  tokens: Array<Token>;
}

export type CreateTextAnnotationBody = Pick<TextAnnotation, 'content' | 'title'>;

export type UpdateTextAnnotationBody = Partial<Pick<TextAnnotation, 'title'>>;
