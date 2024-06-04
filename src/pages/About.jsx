import React, { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';

export default function About() {
  const [rssItems, setRssItems] = useState([]);
  const [errors, setErrors] = useState([]);

  useEffect(() => {
    const unlistenRss = listen('new-rss-items', (event) => {
      setRssItems(event.payload);
    });

    const unlistenError = listen('feed-error', (event) => {
      setErrors((prevErrors) => [...prevErrors, event.payload]);
    });

    return () => {
      unlistenRss.then((fn) => fn());
      unlistenError.then((fn) => fn());
    };
  }, []);

  return (
    <>
      <h1>Temp render of new feed items</h1>
      <ul>
        {rssItems.map((item, index) => (
          <li key={index}>
            <a href={item.link} target="_blank" rel="noopener noreferrer">
              {item.title}
            </a>
            <p>{item.description}</p>
          </li>
        ))}
      </ul>
      {errors.length > 0 && (
        <div>
          <h2>Errors:</h2>
          <ul>
            {errors.map((error, index) => (
              <li key={index}>{error}</li>
            ))}
          </ul>
        </div>
      )}
    </>
  );
}
