import { useNavigate } from "react-router-dom"
import Sidebar from '../components/general/Sidebar';

export default function About() {
    const navigate = useNavigate();
    return (
            <button onClick={() => navigate("/", { replace: true })}> Click this to go back</button>
      );
}