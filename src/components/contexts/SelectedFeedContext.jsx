import React, { createContext, useState } from 'react';

export const SelectedFeedContext = createContext();

export const SelectedFeedProvider = ({ children }) => {
  const [selectedFeed, setSelectedFeed] = useState(null);

  return (
    <SelectedFeedContext.Provider value={{ selectedFeed, setSelectedFeed }}>
      {children}
    </SelectedFeedContext.Provider>
  );
};

