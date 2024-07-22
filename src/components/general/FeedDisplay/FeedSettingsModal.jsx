import React, { useState, useEffect } from 'react';
import styles from './FeedDisplay.module.css';

export default function FeedSettingsModal({ onClose, feed }) {
  const [feedData, setFeedData] = useState({ alias: '', url: '', pollTimer: '' });

  useEffect(() => {
    if (feed) {
      setFeedData({
        alias: feed.alias || '',  // Default to empty string if undefined
        url: feed.url || '',      // Default to empty string if undefined
        pollTimer: feed.poll_interval || '' // Default to empty string if undefined
      });
    }
  }, [feed]);

  const handleChange = (e) => {
    setFeedData({
      ...feedData,
      [e.target.name]: e.target.value,
    });
  };

  const handleUpdate = () => {
    // Handle update logic here
    console.log('Update feed', feedData);
    onClose();
  };

  const handleDelete = () => {
    // Handle delete logic here
    console.log('Delete feed', feed.url);
    onClose();
  };

  return (
    <div className={styles.modal}>
      <div className={styles.modalContent}>
        <span className={styles.close} onClick={onClose}>&times;</span>
        <form>
          <label>
            Feed Alias:
            <input 
              type="text" 
              name="alias" 
              value={feedData.alias} 
              onChange={handleChange} 
            />
          </label>
          <label>
            Feed Url:
            <input 
              type="text" 
              name="url" 
              value={feedData.url} 
              onChange={handleChange} 
            />
          </label>
          <label>
            Poll Timer:
            <input 
              type="text" 
              name="pollTimer" 
              value={feedData.pollTimer} 
              onChange={handleChange} 
            />
          </label>
        </form>
        <div className={styles.modalActions}>
          <button onClick={handleUpdate}>Update</button>
          <button onClick={handleDelete}>Delete</button>
        </div>
      </div>
    </div>
  );
}
