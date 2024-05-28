import { invoke } from '@tauri-apps/api'
import React, { useState } from 'react';

function Message() {
    const [greeting, setGreeting] = useState('');  // `invoke` returns a Promise
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
        <div>
            <h1>Hello From Message Comp</h1>
            <p>{greeting}</p>
        </div>
    )
}
export default Message;