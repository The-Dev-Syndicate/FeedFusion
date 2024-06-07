import React from 'react';
import { createRoot } from 'react-dom/client';
import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import App from './App';
import './App.css' // TODO: When a css framework is picked we can add that import here
import About from './pages/About';
import Article from './components/general/Article';
import Layout from './components/PageLayouts/Base'; // If you look at BaseLayout you will see we default export Base so we can just import under any name we want
import NotFound from './pages/NotFound';
import { FeedProvider } from './components/contexts/FeedProvider';

const router = createBrowserRouter([
  {
    path: '/',
    element: <Layout />,  // Use Layout component here
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
        path: '/article/:id',
        element: <Article />
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
    <React.StrictMode>
      <RouterProvider router={router} />
    </React.StrictMode>
  </FeedProvider>
);
