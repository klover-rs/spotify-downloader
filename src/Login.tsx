import { FormEvent, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

import "./Login.css";

export default function Login() {
  const nav = useNavigate();

  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    try {
      await invoke("login", {
        username: username,
        password: password,
      });

      nav("/", { replace: true });
    } catch (e) {
      console.error(e);
      setError(String(e));
    }
  };

  return (
    <div className="login-container">
      <div className="login-container-inner">
        <form className="login-form" onSubmit={handleSubmit}>
          <div className="username-container">
            <label>Username</label>
            <br />
            <input
              type="text"
              placeholder="username here"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
            />
            <br />
          </div>
          <div className="password-container">
            <label>Password</label>
            <br />
            <input
              type="password"
              placeholder="password here"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
            />
            <br />
          </div>
          <input className="submit-btn" type="submit" />
        </form>
        <div>
          <h4>{error}</h4>
        </div>
      </div>
    </div>
  );
}
