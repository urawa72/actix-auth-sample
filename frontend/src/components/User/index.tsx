import { useUser } from './hooks';

const User = () => {
  const { data, loading, error } = useUser();

  if (loading) {
    return <p>Loading...</p>;
  }

  if (error) {
    return <p>Error!!!</p>;
  }

  return (
    <div>
      <p>{data?.username}</p>
      <p>{data?.email}</p>
    </div>
  );
};

export default User;
