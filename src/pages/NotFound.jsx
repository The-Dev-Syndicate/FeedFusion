import { useNavigate } from "react-router-dom";

export default function NotFound(){
    const navigate = useNavigate();
    return (
        <div>
            <h1>404 - Page Not Found</h1>
            <p>I've no idea how you got here but this page does not exist.</p>
            <p>Please use the below nav to find a correct page!</p>
            <small>Or use this </small><button className="inline-button" onClick={() => navigate('/', {replace: true})}>button</button><small> to just go home...</small>
        </div>
    );
}
