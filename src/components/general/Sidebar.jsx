import React from 'react';
import '../../App.css';
import FeedList from './FeedList';

export default function Sidebar() {
  return (
    <div className="sidebar">
      <div className="feed-list">
        <h3>Feeds</h3>
          <FeedList /> {/* TODO: this component needs to interact with rust*/}

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

