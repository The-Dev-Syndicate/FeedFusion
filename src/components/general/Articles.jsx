// src/pages/Articles.js
import React, { useContext, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import ArticleCard from '../general/ArticleCard';
import { RssItemsContext, ErrorsContext } from '../contexts/FeedProvider';
import { SelectedFeedContext } from '../contexts/SelectedFeedContext';
import ArticleModal from './ArticleModal'; // Import the new ArticleModal component

export default function Articles() {
  const navigate = useNavigate();
  const { rssItems } = useContext(RssItemsContext);
  const { errors } = useContext(ErrorsContext);
  const { selectedFeed } = useContext(SelectedFeedContext);
  const [selectedArticle, setSelectedArticle] = useState(null); // Add state for selected article

  const handleCardClick = (hash) => {
    console.log('Clicked index:', hash);
    const article = rssItems.find(item => {
      const rssHash = item.RSS ? String(item.RSS.hash) : null;
      const atomHash = item.ATOM ? String(item.ATOM.hash) : null;
      
      console.log('Checking item:', { rssHash, atomHash });

      return (item.RSS && rssHash === hash) || (item.ATOM && atomHash === hash);
    });

    if (article) {
      setSelectedArticle(article);
    } else {
      console.log('Article not found for hash:', hash);
    }
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

  const filteredItems = selectedFeed
    ? rssItems.filter((article) => article.RSS?.link?.startsWith(selectedFeed) || article.ATOM?.link?.startsWith(selectedFeed))
    : rssItems;

  return (
    <div className="articles-container">
      {filteredItems.map((article, index) => (
        <div key={index} onClick={() => handleCardClick(article.RSS ? `${article.RSS.hash}` : `${article.ATOM.hash}`)}>
          <ArticleCard
            title={article.RSS ? article.RSS.title : article.ATOM.title}
            date={article.RSS ? article.RSS.pub_date : article.ATOM.pub_date}
            author={article.RSS ? article.RSS.author : article.ATOM.author}
            description={article.RSS ? getFirstSentence(article.RSS.description) : article.ATOM.summary}
            onClick={() => handleCardClick(article.RSS ? `${article.RSS.hash}` : `${article.ATOM.hash}`)}
          />
        </div>
      ))}
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
      {selectedArticle && <ArticleModal article={selectedArticle} onClose={() => setSelectedArticle(null)} />}
    </div>
  );
}
