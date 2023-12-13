import { Button, Card, Text, Tooltip } from '@mantine/core';
import { TextAnnotation } from '../../types/annotations';
import { Link } from 'react-router-dom';
import { useContext, useState } from 'react';
import { DashboardContext } from '../../contexts/Dashboard.context';
import ConfirmModal from '../modal/confirm.modal';

export interface AnnotationCardProps {
  data: TextAnnotation;
}

const TextAnnotationCard = ({ data }: AnnotationCardProps) => {
  const { deleteTextAnnotation } = useContext(DashboardContext);

  const [showDelete, setShowDelete] = useState(false);

  return (
    <>
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
            <Button onClick={() => setShowDelete(true)}>Delete</Button>
            <Link to={`/dashboard/text/${data._id.$oid}`}>
              <Button>Edit</Button>
            </Link>
          </div>
        </div>
      </Card>
      <ConfirmModal
        title="Are you sure you want to delete this annotation ?"
        opened={showDelete}
        onAccept={() => deleteTextAnnotation(data._id.$oid)}
        onClose={() => setShowDelete(false)}
      />
    </>
  );
};

export default TextAnnotationCard;
