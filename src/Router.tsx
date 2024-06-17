import { MemoryRouter as Router, Routes, Route } from 'react-router-dom';
import App from './App';
import Login from './Login';
import './style.css';

export default function DomRouter() {
    return (
        <Router>
            <Routes>
                <Route path='/' element={<App />} />
                <Route path='/login' element={<Login />} />
            </Routes>
        </Router>
    )
}