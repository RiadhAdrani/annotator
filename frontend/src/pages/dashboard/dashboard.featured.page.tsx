import { Button, Title } from '@mantine/core';
import { useContext, useState } from 'react';
import CreateTextAnnotationModal from '../../components/dashboard/createTextAnnotation.modal';
import { DashboardContext } from '../../contexts/Dashboard.context';
import TextAnnotationCard from '../../components/cards/annotation.card';

const DashboardFeaturedPage = () => {
  const { textAnnotations } = useContext(DashboardContext);

  const [showTextModal, setShowTextModal] = useState(false);

  return (
    <>
      <div className="col gap-3">
        <div>
          <div className="row items-center gap-5">
            <Title>Text Annotation</Title>
            <Button onClick={() => setShowTextModal(true)}>Create</Button>
            <CreateTextAnnotationModal
              opened={showTextModal}
              onClose={() => setShowTextModal(false)}
            />
          </div>
          <div className="grid grid-cols-4 gap-4">
            {textAnnotations.map((it) => (
              <TextAnnotationCard key={it._id.$oid} data={it} />
            ))}
          </div>
        </div>
        <div className="row gap-3">
          <Button disabled>Document annotation</Button>
          <Button disabled>Image annotation</Button>
        </div>
      </div>
    </>
  );
};

export default DashboardFeaturedPage;
