import FeedList from './components/general/FeedList';
import Message from './components/general/Message';
import { useNavigate } from "react-router-dom";

function App() {
  const navigate = useNavigate()
  return (
    <Message />
  );
}
export default App;