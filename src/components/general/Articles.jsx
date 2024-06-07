import React, { useContext } from 'react';
import { useNavigate } from 'react-router-dom';

import ArticleCard from '../general/ArticleCard';
import { RssItemsContext, ErrorsContext } from '../contexts/FeedProvider';

export default function Articles() {
  const navigate = useNavigate();
  const { rssItems } = useContext(RssItemsContext); // Use context directly, no need to destructure
  const { errors } = useContext(ErrorsContext); // Use context directly, no need to destructure

  const handleCardClick = (id) => {
    console.log('Clicked index:', id); // Debug log
    navigate(`/article/${id}`);
  };

  return (
    <div className="articles-container">
      {rssItems.map((article, index) => (
        <div key={index} onClick={() => handleCardClick(index)}>
          <ArticleCard
            title={article.title}
            date={article.date}
            author={article.author}
            description={article.description}
          />
        </div>))}
      {errors.length > 0 && (
        <div>
          <h2>Errors:</h2>
          <ul>
            {errors.map((error, index) => (
              <li key={index}>{error}</li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}
