// This component kicks off the listeners for rust events from feeds and then what ever comp. needs the feed info can grab it
import React, { useEffect, useState, createContext } from 'react';
import { listen } from '@tauri-apps/api/event';

export const RssItemsContext = createContext(); // Create context for RSS items
export const ErrorsContext = createContext(); // Create context for errors

export const FeedProvider = ({ children }) => {
  const [rssItems, setRssItems] = useState([]);
  const [errors, setErrors] = useState([]);

  useEffect(() => {
    const unlistenRss = listen('new-rss-items', (event) => {
      const newItems = event.payload.filter(newItem => {
        // Create the special key for the new item
        const newItemKey = `${newItem.title}-${newItem.author}-${newItem.pub_date}-${newItem.hash}`;

        // Check if an item with the same key already exists in rssItems
        const exists = rssItems.some(item => {
          const itemKey = `${item.title}-${item.author}-${item.pub_date}-${newItem.hash}`;
          return itemKey === newItemKey;
        });
        // Only add the new item if it doesn't already exist
        return !exists;
      });
      // Update rssItems with the new items
      setRssItems((prevItems) => [...prevItems, ...newItems]);
    });

    const unlistenError = listen('feed-error', (event) => {
      setErrors((prevErrors) => [...prevErrors, event.payload]);
    });

    return () => {
      unlistenRss.then((fn) => fn());
      unlistenError.then((fn) => fn());
    };
  }, [rssItems]);

  return (
    <RssItemsContext.Provider value={{ rssItems, setRssItems }}>
      <ErrorsContext.Provider value={{ errors, setErrors }}>
        {children}
      </ErrorsContext.Provider>
    </RssItemsContext.Provider>
  );
};
