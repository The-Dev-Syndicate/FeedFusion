import React, { useState, useEffect } from 'react';
import Article from './ArticleCard';
import { invoke } from '@tauri-apps/api';

function Articles() {
  const [articles, setArticles] = useState([]);

  const fetchArticles = async () => {
    try {
      const response = await invoke('get_articles');
      setArticles(response);
    } catch (error) {
      console.error('Error fetching articles:', error);
    }
  };

  useEffect(() => {
    fetchArticles();
  }, []);

  return (
    <div className="articles-container">
      {articles.map((article, index) => (
        <Article
          key={index}
          title={article.title}
          description={article.description}
          author={article.author}
          datetime={article.datetime}
        />
      ))}
    </div>
  );
}

export default Articles;
