import React, { useState } from 'react';
import '../../App.css';
import FeedList from './FeedDisplay/FeedList';
import AddFeedModal from './AddFeedModal';


export default function Sidebar() {
  const [isModalOpen, setIsModalOpen] = useState(false);

  const openModal = () => {
    setIsModalOpen(true);
  };

  const closeModal = () => {
    setIsModalOpen(false);
  };


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
          <span>Add Feed</span>
        </div>
        <button className="add-feed circle-button" onClick={openModal}>+</button>
        <AddFeedModal isOpen={isModalOpen} onClose={closeModal} />
      </div>
    </div>
  );
};

