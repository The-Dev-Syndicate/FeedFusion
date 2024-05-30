import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api';

export default function AddFeedModal({ isOpen, onClose }) {
  const [url, setUrl] = useState('');
  const [name, setName] = useState('');
  const [pollTimer, setPollTimer] = useState(10); // default to 10 minutes

  if (!isOpen) {
    return null;
  }

  const handleSubmit = (e) => {
    e.preventDefault();
    // Perform form validation and submission logic here
    console.log({ url, name, pollTimer });
    invoke('add_feed_url', { feedUrl: url })
    .then(() => {
      console.log("Feed URL added successfully");
      setUrl('');
      setName('');
      setPollTimer(10);
      onClose();
    })
    .catch((error) => {
      console.error("Failed to add feed URL", error);
    });

    // Reset form and close modal
    setUrl('');
    setName('');
    setPollTimer(10);
    onClose();
  };

  return (
    <div className="modal">
      <div className="modal-content">
        <span className="close" onClick={onClose}>&times;</span>
        <h2>Add New Feed</h2>
        <form onSubmit={handleSubmit}>
          <div>
            <label>URL (required):</label>
            <input
              type="text"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
              required
            />
          </div>
          <div>
            <label>Name (optional):</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </div>
          <div>
            <label>Poll Timer (minutes, optional):</label>
            <input
              type="number"
              value={pollTimer}
              onChange={(e) => setPollTimer(e.target.value)}
              min="1"
            />
          </div>
          <button type="submit">Add Feed</button>
        </form>
      </div>
    </div>
  );
}

