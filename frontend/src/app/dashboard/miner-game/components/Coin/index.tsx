import React from 'react'

import './index.css';

interface ICoin {}

const Coin: React.FC<ICoin> = (props) => {
  return (
    <div
      className="coin-sprite"
      style={{
        backgroundImage: 'url(/rewards/gold.png)'
      }}
    >
    </div>
  )
};

export default Coin;
