import React from 'react';
import { FiSettings } from 'react-icons/fi'; // Using react-icons for the cog icon
import styles from './FeedDisplay.module.css';

export default function FeedItem({ feed, onOpenModal }) {
  return (
    <li className={styles.listItem}>
      {feed.alias ?? feed.url}
      <FiSettings className="cog-icon" onClick={(e) => {
        e.stopPropagation(); // Prevent triggering the list item click event
        onOpenModal(feed);
      }} />
    </li>
  );
}
