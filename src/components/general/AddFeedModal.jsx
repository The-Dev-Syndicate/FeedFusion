import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api';

export default function AddFeedModal({ isOpen, onClose }) {
  const [url, setUrl] = useState('');
  const [alias, setAlias] = useState('');
  const [pollTimer, setPollTimer] = useState(10); // default to 10 minutes

  if (!isOpen) {
    return null;
  }

  const handleSubmit = (e) => {
    e.preventDefault();
    // Perform form validation and submission logic here
    console.log({ url, name: alias, pollTimer });
    invoke('add_feed', { feedUrl: url, feedAlias: alias, pollTimer })
    .then(() => {
      console.log("Feed URL added successfully");
      setUrl('');
      setAlias('');
      setPollTimer(10);
      onClose();
    })
    .catch((error) => {
      console.error("Failed to add feed URL", error);
    });
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
              value={alias}
              onChange={(e) => setAlias(e.target.value)}
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

