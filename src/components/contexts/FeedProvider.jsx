import React, { useEffect, useState, createContext } from 'react';
import { listen } from '@tauri-apps/api/event';

export const RssItemsContext = createContext(); // Create context for RSS items
export const ErrorsContext = createContext(); // Create context for errors

export const FeedProvider = ({ children }) => {
  const [rssItems, setRssItems] = useState([]);
  const [errors, setErrors] = useState([]);

  useEffect(() => {
    const unlistenRss = listen('new-rss-items', (event) => {
      console.log("Got New Item: ", event.payload)
      setRssItems(event.payload)
      return}
    );

    const unlistenError = listen('feed-error', (event) => {
      setErrors((prevErrors) => [...prevErrors, event.payload]);
    });

    return () => {
      unlistenRss.then((fn) => fn());
      unlistenError.then((fn) => fn());
    };
  }, []);

  return (
    <RssItemsContext.Provider value={{ rssItems, setRssItems }}>
      <ErrorsContext.Provider value={{ errors, setErrors }}>
        {children}
      </ErrorsContext.Provider>
    </RssItemsContext.Provider>
  );
};
