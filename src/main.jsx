import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import './App.css' // TODO: When a css framework is picked we can add that import here

createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
