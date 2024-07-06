import React, { useEffect, useState, useContext } from 'react';
import { invoke } from '@tauri-apps/api';
import { SelectedFeedContext } from '../contexts/SelectedFeedContext';

export default function FeedList() {
  const [feeds, setFeeds] = useState([]);
  const { selectedFeed, setSelectedFeed } = useContext(SelectedFeedContext);

  useEffect(() => {
    invoke('load_feeds')
      .then((items) => {
        const uniqueFeeds = removeDuplicates(feeds.concat(items));
        setFeeds(uniqueFeeds);
      })
      .catch(console.error);
  }, [feeds]); // Include feeds in the dependency array

  const removeDuplicates = (arr) => {
    const uniqueMap = new Map();
    arr.forEach((feed) => {
      uniqueMap.set(feed.url, feed); // Ensure URL is used as the key for uniqueness
    });
    return Array.from(uniqueMap.values());
  };

  const handleFeedClick = (feedUrl) => {
    if (selectedFeed === feedUrl) {
      setSelectedFeed(null); // Clear the filter
    } else {
      setSelectedFeed(feedUrl); // Set the selected feed
    }
  };

  return (
    <ul>
      {feeds.map((feed, index) => (
        <li key={index} className={`list-item ${selectedFeed === feed.url ? 'selected' : ''}`} onClick={() => handleFeedClick(feed.url)}>
          {feed.alias ?? feed.url}
        </li>
      ))}
    </ul>
  );
}
