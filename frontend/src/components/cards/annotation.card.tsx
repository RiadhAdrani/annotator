import { Button, Card, Text, Tooltip } from '@mantine/core';
import { TextAnnotation } from '../../types/annotations';
import { Link } from 'react-router-dom';

export interface AnnotationCardProps {
  data: TextAnnotation;
}

const TextAnnotationCard = ({ data }: AnnotationCardProps) => {
  return (
    <Card shadow="sm" h={275}>
      <div className="col h-full gap-4 p-2">
        <Tooltip label={data.title}>
          <Text size="xl" truncate={'end'}>
            {data.title}
          </Text>
        </Tooltip>
        <div className="flex-1">
          <Text lineClamp={4}>{data.content.substring(0, 200)}</Text>
        </div>
        <div className="row gap-2 justify-end">
          <Button>Delete</Button>
          <Link to={`/dashboard/text/${data._id.$oid}`}>
            <Button>Edit</Button>
          </Link>
        </div>
      </div>
    </Card>
  );
};

export default TextAnnotationCard;
