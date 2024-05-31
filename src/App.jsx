import FeedList from './components/general/FeedList';
import Articles from './components/general/Articles';
import { useNavigate } from "react-router-dom";

function App() {
  const navigate = useNavigate()
  return (
    <Articles />
  );
}
export default App;