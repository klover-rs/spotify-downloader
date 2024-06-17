import { FormEvent, useState } from "react";
import { invoke } from '@tauri-apps/api/tauri';
import { useNavigate } from "react-router-dom";

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
                password: password
            });

            nav("/", { replace: true });
        } catch (e) {
            console.error(e);
            setError(String(e));
        }
    } 

    return (
        <div>
            <form onSubmit={handleSubmit}>
                <label>Username</label>
                <br/>
                <input 
                    type="text"
                    placeholder="username here"
                    value={username}
                    onChange={(e) => setUsername(e.target.value)}
                />
                <br/>
                <label>Password</label>
                <br/>
                <input
                    type='password'
                    placeholder="password here"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                />
                <br/>
                <input type="submit"/>
            </form>
            <div>
                <h4>{error}</h4>
            </div>
        </div>
    );
}