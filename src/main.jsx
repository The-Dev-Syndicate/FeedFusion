import React from 'react';
import { createRoot } from 'react-dom/client';
import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import App from './App';
import './App.css'; 
import About from './pages/About';
import Layout from './components/PageLayouts/Base';
import NotFound from './pages/NotFound';
import { FeedProvider } from './components/contexts/FeedProvider';
import { SelectedFeedProvider } from './components/contexts/SelectedFeedContext';

const router = createBrowserRouter([
  {
    path: '/',
    element: <Layout />,
    children: [
      {
        path: '/',
        element: <App />
      },
      {
        path: '/about',
        element: <About />
      },
      {
        path: '*',
        element: <NotFound />
      }
    ]
  }
]);

createRoot(document.getElementById('root')).render(
  <FeedProvider>
    <SelectedFeedProvider> 
      <React.StrictMode>
        <RouterProvider router={router} />
      </React.StrictMode>
    </SelectedFeedProvider>
  </FeedProvider>
);

export default App;
