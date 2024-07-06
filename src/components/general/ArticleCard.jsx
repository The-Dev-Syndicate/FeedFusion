// src/general/ArticleCard.js
import React from 'react';

function ArticleCard({ title, description, author, date, onClick }) {
  return (
    <div className="article-card" onClick={onClick}>
      <h2>{title}</h2>
      <p>{description && description.length > 100 ? description.substring(0, 100) + '...' : description}</p>
      <span className="datetime">{date}</span>
      <span className="author-info">{author}</span>
    </div>
  );
}

export default ArticleCard;
