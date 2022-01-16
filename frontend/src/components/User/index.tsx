import { useUser } from './hooks';

const User = () => {
  const { data, isLoading } = useUser();

  if (isLoading) {
    return <p>Loading...</p>;
  }

  return (
    <div>
      <p>{data?.username}</p>
      <p>{data?.email}</p>
    </div>
  );
};

export default User;
