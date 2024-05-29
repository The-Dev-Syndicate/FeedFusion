import FeedList from './components/general/FeedList';
import Message from './components/general/Message';
import { useNavigate } from "react-router-dom";
import Sidebar from './components/general/Sidebar';

function App() {
  const navigate = useNavigate()
  return (
    <div className='container'>
      <Sidebar />
      <div className='content'>
        <Message />
        <FeedList />
        <button onClick={() => navigate("/about", { replace: true })}> Click this to go back</button>
      </div>
    </div>
  );
}
export default App;