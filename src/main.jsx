// App.js (or your main entry point)
import React from 'react';
import { createRoot } from 'react-dom/client';
import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import App from './App';
import './App.css'; // Add your CSS imports here
import About from './pages/About';
import Layout from './components/PageLayouts/Base';
import NotFound from './pages/NotFound';
import { FeedProvider } from './components/contexts/FeedProvider';
import { SelectedFeedProvider } from './components/contexts/SelectedFeedContext'; // Correct import for SelectedFeedProvider

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
    <SelectedFeedProvider> {/* Wrap your SelectedFeedProvider here */}
      <React.StrictMode>
        <RouterProvider router={router} />
      </React.StrictMode>
    </SelectedFeedProvider>
  </FeedProvider>
);

export default App;
