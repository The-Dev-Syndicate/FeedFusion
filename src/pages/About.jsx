import React, { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';


export default function About() {

      const [rssItems, setRssItems] = useState([]);
      useEffect(() => {
            const unlisten = listen('new-rss-items', (event) => {
                  setRssItems(event.payload);
            });

            return () => {
                  unlisten.then((fn) => fn());
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

            </>
      );
}