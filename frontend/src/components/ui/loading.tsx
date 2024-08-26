import React from 'react'
import LoadingSVG from '@/components/icon/loading.svg';

const Loading: React.FC = () => {
  return (
    <div className="flex items-center justify-center animate-spin">
      <LoadingSVG className='scale-y-[-1]' height={'40px'} width={'40px'} /> 
    </div>
  )
};

export default Loading;
