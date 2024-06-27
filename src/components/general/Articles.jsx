import React, { useContext } from 'react';
import { useNavigate } from 'react-router-dom';

import ArticleCard from '../general/ArticleCard';
import { RssItemsContext, ErrorsContext } from '../contexts/FeedProvider';

export default function Articles() {
  const navigate = useNavigate();
  const { rssItems } = useContext(RssItemsContext); // Use context directly, no need to destructure
  const { errors } = useContext(ErrorsContext); // Use context directly, no need to destructure

  console.log(rssItems);

  const handleCardClick = (title) => {
    console.log('Clicked index:', title); // Debug log
    navigate(`/article/${title}`);
  };

  function getFirstSentence(htmlString) {
    const div = document.createElement('div');
    div.innerHTML = htmlString;
    const firstParagraph = div.querySelector('p');
    if (firstParagraph) {
      const sentences = firstParagraph.innerText.split('. ');
      return sentences.length > 0 ? sentences[0] + '.' : '';
    }
    return '';
  }

  return (
    <div className="articles-container">
      {rssItems.map((article, index) => (
        <div key={index} onClick={() => handleCardClick(article.Rss ? `${index}-${article.Rss.title}` : article.Atom.title)}>
          <ArticleCard
            title={article.Rss ? article.Rss.title : article.Atom.title}
            date={article.Rss ? article.Rss.pub_date : article.Atom.pub_date}
            author={article.Rss ? article.Rss.author : article.Atom.author}
            description={article.Rss ? getFirstSentence(article.Rss.description) : article.Atom.summary}
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
