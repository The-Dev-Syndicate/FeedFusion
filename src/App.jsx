import logo from './logo.svg';
import './App.css';
import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api'

function App() {
  const [greeting, setGreeting] = useState('');
  // Function to call the greet command and update the state
  const getGreeting = async (name) => {
    try {
      const response = await invoke('greet', { name });
      setGreeting(response);
    } catch (error) {
      console.error('Error invoking greet command:', error);
    }
  };
  // Call the function when the component mounts
  React.useEffect(() => {
    getGreeting('(TESTING RUST FUNCTION CALL)');
  }, []);
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Hello World from <code>src/App.js</code>
        </p>
        <p>{greeting}</p>

        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
