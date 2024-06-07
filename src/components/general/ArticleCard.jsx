import React from 'react';

function ArticleCard({ title, description, author, date }) {
  return (
    <div className="article-card">
      <h2>{title}</h2>
      <p>{description}</p>
      <span className="datetime">{date}</span>
      <span className="author-info">{author}</span>
    </div>
  );
}

export default ArticleCard;
