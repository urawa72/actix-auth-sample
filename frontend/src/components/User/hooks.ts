import { useEffect, useState } from 'react';

interface UserInfo {
  username: string;
  email: string;
}

export const useUser = () => {
  const [data, setData] = useState<UserInfo>();
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    setIsLoading(true);
    fetch('http://localhost:8000/user/info')
      .then((res) => res.json())
      .then((data) => {
        setData(data);
        setIsLoading(false);
      })
      .catch((e) => {
        console.log(e);
        setIsLoading(false);
      });
  }, []);

  return { data, isLoading };
};
