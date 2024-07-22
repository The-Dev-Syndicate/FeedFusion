import React, { useEffect, useState, useContext } from 'react';
import { invoke } from '@tauri-apps/api';
import { SelectedFeedContext } from '../../contexts/SelectedFeedContext';
import FeedItem from './FeedItem';
import FeedSettingsModal from './FeedSettingsModal';
import styles from './FeedDisplay.module.css';

export default function FeedList() {
  const [feeds, setFeeds] = useState([]);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [currentFeed, setCurrentFeed] = useState(null); // State for the currently selected feed
  const { selectedFeed, setSelectedFeed } = useContext(SelectedFeedContext);

  useEffect(() => {
    invoke('load_feeds')
      .then((items) => {
        const uniqueFeeds = removeDuplicates(feeds.concat(items));
        setFeeds(uniqueFeeds);
      })
      .catch(console.error);
  }, [feeds]);

  const removeDuplicates = (arr) => {
    const uniqueMap = new Map();
    arr.forEach((feed) => {
      uniqueMap.set(feed.url, feed);
    });
    return Array.from(uniqueMap.values());
  };

  const handleOpenModal = (feed) => {
    console.log(feed)
    setCurrentFeed(feed); // Set the current feed to be edited
    setIsModalOpen(true);
  };

  const handleCloseModal = () => {
    setIsModalOpen(false);
    setCurrentFeed(null); // Clear the current feed when closing the modal
  };

  const handleShowAll = () => {
    setSelectedFeed(null); // Clear the filter to show all feeds
  };

  const filteredFeeds = selectedFeed
    ? feeds.filter(feed => feed.url === selectedFeed)
    : feeds;

  return (
    <>
      <ul>
        <li className={styles.listItem} onClick={handleShowAll}>All</li>
        {filteredFeeds.map((feed, index) => (
          <FeedItem key={index} feed={feed} onOpenModal={handleOpenModal} />
        ))}
      </ul>
      {isModalOpen && <FeedSettingsModal onClose={handleCloseModal} feed={currentFeed} />}
    </>
  );
}
