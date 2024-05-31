import React from 'react';

function ArticleCard({ title, description, author, datetime }) {
  return (
    <div className="article-card">
      <h2>{title}</h2>
      <p>{description}</p>
      <span className="datetime">{datetime}</span>
      <span className="author-info">{author}</span>
    </div>
  );
}

export default ArticleCard;
