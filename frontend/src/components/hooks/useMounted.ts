import { useLayoutEffect, useState } from "react";

export const useMounted = () => {
  const [mounted, setMounted] = useState<boolean>(false);

  useLayoutEffect(() => {
    setMounted(true);
  }, []);
  return mounted;
};
