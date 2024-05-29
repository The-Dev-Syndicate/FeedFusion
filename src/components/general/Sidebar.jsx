import React from 'react';
import '../../App.css';

export default function Sidebar() {
  return (
    <div className="sidebar">
      <div className="feed-list">
        <h3>Feeds</h3>
        {/* Add your feed items here */}
      </div>
      <div className="settings">
        <div className="icons">
          {/* Add your settings icons here */}
          <span>Settings</span>
        </div>
        <button className="add-feed">+</button>
      </div>
    </div>
  );
};

