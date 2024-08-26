// components/NoData.js
import React from "react";
import NoDataSvg from "@/components/icon/no-data.svg";
const NoData = () => {
  return (
    <div className="flex items-center justify-center w-full h-[200px]">
      <div className="flex flex-col items-center justify-center text-center">
        <NoDataSvg className="opacity-30" />
        <h2 className="text-2xl font-bold mt-4 mb-4">No Data Available</h2>
        <p className="text-gray-500">
          Sorry, there is no data to display at the moment.
        </p>
      </div>
    </div>
  );
};

export default NoData;
