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
      const rssHash = item.Rss ? String(item.Rss.hash) : null;
      const atomHash = item.Atom ? String(item.Atom.hash) : null;
      
      console.log('Checking item:', { rssHash, atomHash });

      return (item.Rss && rssHash === hash) || (item.Atom && atomHash === hash);
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
    ? rssItems.filter((article) => article.Rss?.link?.startsWith(selectedFeed) || article.Atom?.link?.startsWith(selectedFeed))
    : rssItems;

  return (
    <div className="articles-container">
      {filteredItems.map((article, index) => (
        <div key={index} onClick={() => handleCardClick(article.Rss ? `${article.Rss.hash}` : `${article.Atom.hash}`)}>
          <ArticleCard
            title={article.Rss ? article.Rss.title : article.Atom.title}
            date={article.Rss ? article.Rss.pub_date : article.Atom.pub_date}
            author={article.Rss ? article.Rss.author : article.Atom.author}
            description={article.Rss ? getFirstSentence(article.Rss.description) : article.Atom.summary}
            onClick={() => handleCardClick(article.Rss ? `${article.Rss.hash}` : `${article.Atom.hash}`)}
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
