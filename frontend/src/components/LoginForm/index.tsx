import { useCallback, useState } from 'react';

type IndexResponse = {
  counter: number;
};

const LoginForm = () => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [counter, setCounter] = useState(0);

  const postLogin = useCallback(() => {
    const method = 'POST';
    const headers = {
      Accept: 'application/json',
      'Content-Type': 'application/json',
    };
    const body = JSON.stringify({ user_id: '1' });
    const result = fetch('http://localhost:8000/login', {
      method,
      headers,
      body,
      credentials: 'include',
      mode: 'cors',
    })
      .then((res) => res.json())
      .then((json) => {
        console.log(json);
        setIsAuthenticated(true);
      })
      .catch((e) => console.log(e));
    return result;
  }, []);

  const postLogout = useCallback(() => {
    const method = 'POST';
    const headers = {
      Accept: 'application/json',
      'Content-Type': 'application/json',
    };
    const result = fetch('http://localhost:8000/logout', {
      method,
      headers,
      credentials: 'include',
      mode: 'cors',
    })
      .then((_) => {
        setIsAuthenticated(false);
        setCounter(0);
      })
      .catch((e) => console.log(e));
    return result;
  }, []);

  const doSomething = useCallback(() => {
    const method = 'POST';
    const headers = {
      Accept: 'application/json',
      'Content-Type': 'application/json',
    };
    const result = fetch('http://localhost:8000/do_something', {
      method,
      headers,
      credentials: 'include',
      mode: 'cors',
    })
      .then((res) => res.json())
      .then((json: IndexResponse) => {
        console.log(json);
        setCounter(json.counter);
      })
      .catch((e) => console.log(e));
    return result;
  }, []);

  return (
    <div>
      <div>{isAuthenticated ? 'Authenticated' : 'Not Authenticated'}</div>
      <div>{counter}</div>
      <div>
        <input type="text" placeholder="id" />
      </div>
      <div>
        <input type="password" placeholder="password" />
      </div>
      <div>
        <button onClick={postLogin}>Login</button>
      </div>
      <div>
        <button onClick={postLogout}>Logout</button>
      </div>
      <div>
        <button onClick={doSomething}>Do somthing</button>
      </div>
    </div>
  );
};

export default LoginForm;
