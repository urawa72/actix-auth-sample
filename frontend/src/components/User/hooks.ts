import { useEffect, useState } from 'react';

interface UserInfo {
  username: string;
  email: string;
}

export const useUser = () => {
  const [data, setData] = useState<UserInfo>();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  useEffect(() => {
    setLoading(true);
    fetch('http://localhost:8000/users/info', { credentials: 'include' })
      .then((res) => res.json())
      .then((data) => {
        setData(data);
      })
      .catch((e) => {
        console.log(e);
        setError(e);
      })
      .finally(() => setLoading(false));
  }, []);

  return { data, loading, error };
};
